#![allow(dead_code)]

use crate::crawl::{Crawler, Visitor};
use crate::lang::Lang;
use crate::parse::Noeud;
use ignore::DirEntry;
use madvise::AdviseMemory;
use memmap2::{Mmap, MmapOptions};
use std::marker::PhantomData;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::{fs::File, io::Read, path::Path, sync::Mutex};
use thread_local::ThreadLocal;
use tree_sitter::{Parser, Query};
use tree_sitter_graph::{
    ExecutionConfig, Identifier, NoCancellation, Variables, ast, functions::Functions, graph::Value,
};

static MMAP_MIN_SIZE: usize = 8192;

enum FileBuffer {
    Mapped(Mmap),
    Raw(Vec<u8>),
}

impl FileBuffer {
    fn bytes(&self) -> &[u8] {
        match self {
            FileBuffer::Mapped(mmap) => mmap.as_ref(),
            FileBuffer::Raw(bytes) => &bytes,
        }
    }
}

fn buffered(path: &Path, file_size: usize) -> FileBuffer {
    if file_size > MMAP_MIN_SIZE {
        let file = File::open(path).unwrap();
        let mmap = unsafe {
            match MmapOptions::new().map(&file) {
                Ok(map) => map,
                Err(_) => panic!("failed to mmap"),
            }
        };

        mmap.advise_memory_access(madvise::AccessPattern::Sequential)
            .unwrap();
        return FileBuffer::Mapped(mmap);
    }

    // small enough to read into ram
    let mut file = File::open(path).unwrap();
    let mut buf = vec![0; file_size];
    file.read(&mut buf).unwrap();
    FileBuffer::Raw(buf)
}

#[derive(Debug, Clone)]
struct State {
    // pushes subgraphs from thread to an mpsc queue
    tx: Sender<serde_json::Value>,
}

impl Visitor for State {
    type Item = Vec<Option<serde_json::Value>>;

    fn visit(&self, value: Self::Item) {
        for v in value.into_iter().filter_map(|g| g) {
            self.tx.send(v).expect("failed to send");
        }
    }
}

pub struct Draveur<L: Lang> {
    stanzas: ast::File,
    query: Option<Query>,

    // marker type for provided language
    _phantom: PhantomData<L>,
}

impl<L: Lang + Sync> Draveur<L> {
    pub fn new(stanzas: String) -> Self {
        Self {
            stanzas: L::build_stanzas(stanzas),
            query: None,
            _phantom: PhantomData,
        }
    }

    pub fn waltz(&self, crawler: Crawler) {
        let tls = ThreadLocal::with_capacity(10);
        let (tx, rx) = channel();
        let state = State { tx };

        crawler.crawl(|e| self.parse_file(e, &tls), state);

        //consume the queue
        while let Ok(g) = rx.recv() {
            dbg!(&g);
        }
    }

    fn parse_file(
        &self,
        e: &DirEntry,
        tls: &ThreadLocal<Mutex<Parser>>,
    ) -> Vec<Option<serde_json::Value>> {
        let file_size = e.metadata().unwrap().len() as usize;
        let buf = buffered(e.path(), file_size);
        let bytes = buf.bytes();

        // parse source file
        let tree = {
            let parser_mutex = tls.get_or(|| {
                let mut p = Parser::new();
                p.set_language(&tree_sitter_python::LANGUAGE.into())
                    .unwrap();
                Mutex::new(p)
            });
            let mut parser = parser_mutex.lock().unwrap();
            parser.parse(bytes, None).unwrap()
        };

        let root = Noeud::new(tree.root_node(), &bytes);
        let mut graphs = vec![];

        // TODO: rename self.query
        match self.query.as_ref() {
            Some(query) => {
                let mut hits = root.parse(query);

                while let Some(entry) = hits.next() {
                    for (_, node) in &entry {
                        graphs.push(self.build_node_graph(node, tls));
                    }
                }
            }
            None => graphs.push(self.build_node_graph(&root, tls)),
        }
        graphs
    }

    fn build_node_graph(
        &self,
        node: &Noeud,
        tls: &ThreadLocal<Mutex<Parser>>,
    ) -> Option<serde_json::Value> {
        let mut globals = Variables::new();
        globals
            .add(Identifier::from("global_filename"), "nate".into())
            .unwrap();
        globals
            .add(
                Identifier::from("global_row"),
                Value::Integer(node.node.start_position().row as u32),
            )
            .unwrap();
        globals
            .add(
                Identifier::from("global_column"),
                Value::Integer(node.node.start_position().column as u32),
            )
            .unwrap();

        // parse node sub-tree
        let node_tree = {
            let parser_mutex = tls.get_or(|| {
                let mut p = Parser::new();
                p.set_language(&tree_sitter_python::LANGUAGE.into())
                    .unwrap();
                Mutex::new(p)
            });
            let mut parser = parser_mutex.lock().unwrap();
            parser.parse(&node.bytes(), None).unwrap()
        };

        // https://github.com/tree-sitter/tree-sitter-graph/blob/main/tests/it/execution.rs
        let functions = Functions::stdlib();
        let mut config = ExecutionConfig::new(&functions, &globals).lazy(true);

        let graph = self
            .stanzas
            .execute(&node_tree, node.ctx_as_str(), &mut config, &NoCancellation)
            .expect(node.ctx_as_str());

        println!("{}", graph.pretty_print());

        match graph.node_count() {
            0 => None,
            _ => Some(serde_json::to_value(graph).unwrap()),
        }
    }
}
