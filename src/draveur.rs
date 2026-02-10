use crate::{
    dec_s_expr, stanzas, parse::{Noeud, build_query}
};
use ignore::DirEntry;
use madvise::AdviseMemory;
use memmap2::{Mmap, MmapOptions};
use once_cell::sync::Lazy;
use std::{
    fs::File,
    io::Read,
    path::Path,
    sync::{Arc, Mutex},
    thread,
};
use tree_sitter::Parser;
use tree_sitter_graph::{
    ExecutionConfig, NoCancellation, Variables, ast, functions::Functions
};

static MMAP_MIN_SIZE: usize = 8192;

// Per-thread parser pool
static PARSER_POOL: Lazy<Vec<Arc<Mutex<Parser>>>> = Lazy::new(|| {
    let threads = thread::available_parallelism().map_or(1, |t| t.get());
    let mut parsers = Vec::with_capacity(threads);

    for _ in 0..threads {
        let mut parser = Parser::new();
        let lang = tree_sitter_python::LANGUAGE.into();
        parser.set_language(&lang).unwrap();

        parsers.push(Arc::new(Mutex::new(parser)))
    }
    parsers
});

// struct State;
// impl Visitor for State {
//     type Item = todo!();
//
//     fn visit(self: &std::sync::Arc<Self>, _: Self::Item) {
//         todo!()
//     }
// }
// struct Draveur {
//     state: State,
// }
// impl Draveur {
//     pub fn waltz(&self) {}
// }

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

// FIXME: add proper error handling
// FIXME: maybe replace all this with bufreader?
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

/// Applies to each file in the glob **/*.py
/// builds AST and extracts decorator definitions
pub fn tree_sitter_parse(e: &DirEntry) {
    let file_size = e.metadata().unwrap().len() as usize;
    let buf = buffered(e.path(), file_size);
    let bytes = buf.bytes();

    let thread_id = thread::current().id().as_u64().get();
    let id = thread_id % PARSER_POOL.len() as u64;
    let parser_mutex = PARSER_POOL.get(id as usize).unwrap();
    let mut parser = parser_mutex.lock().unwrap();

    let tree = parser.parse(bytes, None).unwrap();
    let root = Noeud::new(tree.root_node(), &bytes);

    let query = build_query(&dec_s_expr!("workflows.workflow.define", "activity", "foo"));
    let mut hits = root.parse(&query);

    while let Some(entry) = hits.next() {
        for (_, e) in &entry {
            // dbg!(e.node.to_sexp());
            tree_sitter_graph(e, &mut parser);
        }
    }
}

fn tree_sitter_graph(node: &Noeud, parser: &mut Parser) {
    // https://github.com/tree-sitter/tree-sitter-graph/blob/main/tests/it/parser.rs
    let source = stanzas!();

    let tree = parser.parse(&node.bytes(), None).unwrap();

    // https://github.com/tree-sitter/tree-sitter-graph/blob/main/tests/it/execution.rs
    let file = ast::File::from_str(tree_sitter_python::LANGUAGE.into(), &source)
        .expect("Cannot parse file");
    let functions = Functions::stdlib();
    let globals = Variables::new();
    let mut config = ExecutionConfig::new(&functions, &globals).lazy(true);

    let graph = file
        .execute(&tree, node.ctx_as_str(), &mut config, &NoCancellation)
        .unwrap();
    println!("{}", graph.pretty_print());
}
