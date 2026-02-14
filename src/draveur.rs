#![allow(dead_code)]

use crate::{
    crawl::{CrawlOpts, Crawler, DoNothingVisitor, Visitor},
    decorated_objects,
    parse::{Noeud, build_query},
    stanzas,
};
use ignore::DirEntry;
use madvise::AdviseMemory;
use memmap2::{Mmap, MmapOptions};
use std::{collections::HashMap, fs::File, io::Read, path::Path, sync::Mutex};
use thread_local::ThreadLocal;
use tree_sitter::{Parser, Query};
use tree_sitter_graph::{
    ExecutionConfig, Identifier, NoCancellation, Variables, ast, functions::Functions, graph::Value,
};

static MMAP_MIN_SIZE: usize = 8192;

struct State;
impl Visitor for State {
    type Item = ();

    fn visit(self: &std::sync::Arc<Self>, _: Self::Item) {
        todo!()
    }
}

pub struct Draveur {
    state: DoNothingVisitor,
}
impl Draveur {
    pub fn new() -> Self {
        Self {
            state: DoNothingVisitor {},
        }
    }

    pub fn waltz(&self, path: &str) {
        let opts = CrawlOpts::default().path(path).threads(10).add_ext("py");

        let crawler = Crawler::new(opts);

        // these just go inside of the draveur...
        // let query = build_query("(module)@all");
        let query = build_query(&decorated_objects!(
            "workflows.workflow.define",
            "workflows.update",
            "workflows.query",
            "workflows.signal",
            "activity",
            "foo"
        ));
        let stanzas = ast::File::from_str(tree_sitter_python::LANGUAGE.into(), &stanzas!())
            .expect(&stanzas!());

        let tls = ThreadLocal::with_capacity(10);

        crawler.crawl(
            |e| tree_sitter_parse(e, &query, &stanzas, &tls),
            self.state.clone(),
        );
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

// TODO: inject globals into node_graph (e.g file name, rel offset) so we can add them as attributes
pub fn tree_sitter_parse(
    e: &DirEntry,
    query: &Query,
    stanzas: &ast::File,
    tls: &ThreadLocal<Mutex<Parser>>,
) {
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

    // add some globals
    let mut globals = Variables::new();
    globals
        .add(
            Identifier::from("global_filename"),
            e.path().to_str().unwrap().into(),
        )
        .unwrap();

    while let Some(entry) = hits.next() {
        for (_, e) in &entry {
            node_graph(e, stanzas, &mut globals, tls);
        }
    }
}

fn node_graph(
    node: &Noeud,
    stanzas: &ast::File,
    globals: &mut Variables,
    tls: &ThreadLocal<Mutex<Parser>>,
) {
    // reset
    globals.remove(&Identifier::from("global_row"));
    globals.remove(&Identifier::from("global_column"));

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
}
