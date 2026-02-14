use crate::
    parse::Noeud
;
use ignore::DirEntry;
use madvise::AdviseMemory;
use memmap2::{Mmap, MmapOptions};
use std::{fs::File, io::Read, path::Path, sync::Mutex};
use thread_local::ThreadLocal;
use tree_sitter::{Parser, Query};
use tree_sitter_graph::{ExecutionConfig, NoCancellation, Variables, ast, functions::Functions};

static MMAP_MIN_SIZE: usize = 8192;

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

pub fn tree_sitter_parse(
    e: &DirEntry,
    query: &Query,
    stanzas: &ast::File,
    tls: &ThreadLocal<Mutex<Parser>>,
) {
    let file_size = e.metadata().unwrap().len() as usize;
    let buf = buffered(e.path(), file_size);
    let bytes = buf.bytes();

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

    while let Some(entry) = hits.next() {
        for (_, e) in &entry {
            // dbg!(e.node.to_sexp());
            tree_sitter_graph(e, stanzas, tls);
        }
    }
}

fn tree_sitter_graph(node: &Noeud, stanzas: &ast::File, tls: &ThreadLocal<Mutex<Parser>>) {
    let tree = {
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
    let globals = Variables::new();
    let mut config = ExecutionConfig::new(&functions, &globals).lazy(true);

    let graph = stanzas
        .execute(&tree, node.ctx_as_str(), &mut config, &NoCancellation)
        .expect(
            node.ctx_as_str()
        );
    println!("{}", graph.pretty_print());
}
