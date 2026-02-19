use crate::{
    IoErrorKind, Result, crawl::{CrawlOpts, Visitor}, errors::Error, lang::Lang, node::Graph, parse::Noeud
};
use ignore::DirEntry;
use madvise::{AccessPattern, AdviseMemory};
use memmap2::{Mmap, MmapOptions};
use std::env;
use std::marker::PhantomData;
use std::sync::mpsc::{Sender, channel};
use std::thread::available_parallelism;
use std::{fs::File, io::Read, path::Path, sync::Mutex};
use thread_local::ThreadLocal;
use tree_sitter::{Parser, Query};
use tree_sitter_graph::{
    ExecutionConfig, Identifier, NoCancellation, Variables, ast, functions::Functions, graph::Value,
};

static MMAP_MIN_SIZE: usize = 8192;

fn available_threads() -> usize {
    env::var("THREADS")
        .ok()
        .and_then(|val| val.parse().ok())
        .or_else(|| available_parallelism().map(|t| t.get()).ok())
        .unwrap_or(1)
}

enum FileBuffer {
    Mapped(Mmap),
    Raw(Vec<u8>),
}

impl FileBuffer {
    fn bytes(&self) -> &[u8] {
        match self {
            FileBuffer::Mapped(mmap) => mmap.as_ref(),
            FileBuffer::Raw(bytes) => bytes,
        }
    }
}

fn buffered(path: &Path, file_size: usize) -> Result<FileBuffer> {
    if file_size > MMAP_MIN_SIZE {
        let file = File::open(path).map_err(|e| IoErrorKind::open(path, e))?;
        let mmap = unsafe {
            MmapOptions::new()
                .map(&file)
                .map_err(|e| IoErrorKind::mmap(path, e))?
        };
        let _ = mmap.advise_memory_access(AccessPattern::Sequential);

        return Ok(FileBuffer::Mapped(mmap));
    }

    // small enough to read into ram
    let mut file = File::open(path).map_err(|e| IoErrorKind::open(path, e))?;
    let mut buf = vec![0; file_size];
    file.read(&mut buf)
        .map_err(|e| IoErrorKind::read(path, e))?;
    Ok(FileBuffer::Raw(buf))
}

// NOTE: either do this or ThreadLocal<Cell<Value>> and iter after
#[derive(Debug, Clone)]
struct State {
    // pushes subgraphs from thread to an mpsc queue
    tx: Sender<serde_json::Value>,
}

impl Visitor for State {
    type Item = Vec<Option<serde_json::Value>>;

    fn visit(&self, value: Self::Item) {
        for v in value.into_iter().flatten() {
            self.tx.send(v).expect("failed to send");
        }
    }
}

pub struct Draveur<L: Lang> {
    mappings: Vec<(Query, ast::File)>,

    // marker type for provided language
    _phantom: PhantomData<L>,
}

impl<L> Draveur<L>
where
    L: Lang + Sync,
{
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn add(&mut self, cause: String, effect: String) -> Result<&mut Self> {
        let cause = L::build_query(cause)?;
        let effect = L::build_stanzas(effect)?;
        self.mappings.push((cause, effect));
        Ok(self)
    }

    pub fn waltz(&self, path: &str) -> Result<()> {
        let tls = ThreadLocal::with_capacity(10);

        let (tx, rx) = channel();
        let state = State { tx };

        let crawler = CrawlOpts::default()
            .path(path)
            .threads(available_threads())
            .add_lang::<L>()
            .build();

        crawler.crawl(
            |e| match self.parse_file(e, &tls) {
                Ok(res) => res,
                Err(e) => {
                    panic!("{:?}", e);
                }
            },
            state,
        );

        //consume the queue
        while let Ok(g) = rx.recv() {
            let graph = Graph::deser(g)?;
            dbg!(graph);
        }
        Ok(())
    }

    fn parse_file(
        &self,
        entry: &DirEntry,
        tls: &ThreadLocal<Mutex<Parser>>,
    ) -> Result<Vec<Option<serde_json::Value>>> {
        let file_size = entry.metadata()?.len() as usize;
        let buf = buffered(entry.path(), file_size)?;
        let bytes = buf.bytes();

        // parse source file
        let tree = {
            let parser_mutex = tls.get_or_try(|| {
                let mut p = Parser::new();
                p.set_language(&L::language())
                    .map_err(|e| Error::lang::<L>(e))?;

                Ok::<Mutex<Parser>, Error>(Mutex::new(p))
            })?;

            let mut parser = parser_mutex.lock().expect("poisoned");
            parser.parse(bytes, None).ok_or_else(|| Error::Parse)?
        };

        let root = Noeud::new(tree.root_node(), bytes);
        let mut graphs = vec![];

        // iterate over all the capture groups
        for (cause, effect) in &self.mappings {
            // parse candidate nodes
            let hits = root.parse(cause);
            for hit in hits {
                // NOTE: this is where recursion would go;
                // - let mut cur = root, cur = node, self.recursive(n-1, &cur, effect) ...
                for (_group, node) in &hit {
                    graphs.push(Self::build_node_graph(node, effect, entry, tls)?);
                }
            }
        }

        Ok(graphs)
    }

    fn build_node_graph(
        node: &Noeud,
        stanzas: &ast::File,
        entry: &DirEntry,
        tls: &ThreadLocal<Mutex<Parser>>,
    ) -> Result<Option<serde_json::Value>> {
        let mut globals = Variables::new();
        globals
            .add(
                Identifier::from("global_filename"),
                entry.path().display().to_string().into(),
            )
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
            let parser_mutex = tls.get_or_try(|| {
                let mut p = Parser::new();
                p.set_language(&L::language())
                    .map_err(|e| Error::lang::<L>(e))?;

                Ok::<Mutex<Parser>, Error>(Mutex::new(p))
            })?;
            let mut parser = parser_mutex.lock().expect("poisoned");
            parser
                .parse(node.bytes(), None)
                .ok_or_else(|| Error::Parse)?
        };
        // dbg!(node_tree.root_node().to_sexp());
        // println!("{}\n", node.ctx_as_str());

        // https://github.com/tree-sitter/tree-sitter-graph/blob/main/tests/it/execution.rs
        let functions = Functions::stdlib();
        let config = ExecutionConfig::new(&functions, &globals).lazy(true);

        let graph = stanzas
            .execute(&node_tree, node.ctx_as_str(), &config, &NoCancellation)
            .unwrap_or_else(|_| panic!("{}", node.ctx_as_str()));

        if graph.node_count() > 0 {
            // println!("{}", graph.pretty_print());
        }

        match graph.node_count() {
            0 => Ok(None),
            _ => Ok(Some(serde_json::to_value(graph).unwrap())),
        }
    }
}
