#![allow(dead_code)]

use crate::{
    crawl::{CrawlOpts, Crawler, Visitor},
    decorated_objects,
    parse::{Noeud, build_query},
    stanzas,
};
use ignore::DirEntry;
use madvise::AdviseMemory;
use memmap2::{Mmap, MmapOptions};
use std::sync::mpsc::{Sender, channel};
use std::{fs::File, io::Read, path::Path, sync::Mutex};
use thread_local::ThreadLocal;
use tree_sitter::{Parser, Query};
use tree_sitter_graph::{
    ExecutionConfig, Identifier, NoCancellation, Variables, ast, functions::Functions, graph::Value,
};

static MMAP_MIN_SIZE: usize = 8192;

#[derive(Debug)]
struct State {
    // pushes subgraph from thread to an mpsc queue
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

pub struct Draveur {
    query: Query,
    stanzas: ast::File,
}

impl Draveur {
    pub fn new(query: Query, stanzas: ast::File) -> Self {
        Self { query, stanzas }
    }

    pub fn waltz(&self, path: &str) {
        let opts = CrawlOpts::default().path(path).threads(10).add_ext("py");
        let crawler = Crawler::new(opts);

        let tls = ThreadLocal::with_capacity(10); // fixme: use available_cores() or builder opts
        let (tx, rx) = channel();
        let state = State { tx };

        crawler.crawl(|e| parse_file(e, &self.query, &self.stanzas, &tls), state);

        //consume the queue
        while let Ok(g) = rx.recv() {
            //dbg!(&g);
        }
    }
}

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

// TODO: create a new helper struct for queries/stanzas - might be draveur
pub fn parse_file(
    e: &DirEntry,
    query: &Query,
    stanzas: &ast::File,
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
    let mut hits = root.parse(&query);

    let mut graphs = vec![];
    while let Some(entry) = hits.next() {
        for (_, node) in &entry {
            graphs.push(build_node_graph(node, stanzas, tls));
        }
    }
    graphs
}

fn build_node_graph(
    node: &Noeud,
    stanzas: &ast::File,
    tls: &ThreadLocal<Mutex<Parser>>,
) -> Option<serde_json::Value> {
    // reset
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

    let graph = stanzas
        .execute(&node_tree, node.ctx_as_str(), &mut config, &NoCancellation)
        .expect(node.ctx_as_str());

    println!("{}", graph.pretty_print());

    match graph.node_count() {
        0 => None,
        _ => Some(serde_json::to_value(graph).unwrap()),
    }
}
