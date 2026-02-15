//! Traverses file directory ignoring globbed patterns in .gitignore
//! use ignore Walker with configurable threads

use ignore::{DirEntry, WalkBuilder, WalkState};
use std::path::PathBuf;
use std::sync::Arc;

use crate::lang::Lang;

pub trait Visitor: Send + Sync {
    type Item;

    fn visit(&self, item: Self::Item);
}

#[allow(dead_code)]
pub struct DoNothingVisitor;
impl Visitor for DoNothingVisitor {
    type Item = ();

    fn visit(&self, _: Self::Item) {}
}

pub(crate) struct CrawlOpts {
    pub dir: PathBuf,
    pub threads: usize,
    pub allowed_exts: Vec<String>,
}

impl CrawlOpts {
    pub fn path<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.dir = dir.into();
        self
    }
    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }
    pub fn add_lang<L: Lang>(mut self) -> Self {
        self.allowed_exts.push(L::EXT.to_string());
        self
    }
    pub fn build(self) -> Crawler {
        Crawler::new(self)
    }
}

impl Default for CrawlOpts {
    fn default() -> Self {
        Self {
            dir: "./".into(),
            threads: 0,
            allowed_exts: vec![],
        }
    }
}

pub(crate) struct Crawler {
    opts: Arc<CrawlOpts>,
}

impl Crawler {
    pub fn new(opts: CrawlOpts) -> Self {
        Self {
            opts: Arc::new(opts),
        }
    }

    pub fn crawl<F, V, I>(&self, f: F, v: V)
    where
        F: Fn(&DirEntry) -> I + Send + Sync,
        V: Visitor<Item = I>,
    {
        let opts = Arc::clone(&self.opts);
        let f = Arc::new(f);
        let visitor = Arc::new(v);

        WalkBuilder::new(&self.opts.dir)
            .follow_links(false)
            .standard_filters(true)
            .threads(opts.threads)
            .build_parallel()
            .run(|| {
                let opts = Arc::clone(&opts);
                let f = Arc::clone(&f);
                let visitor = Arc::clone(&visitor);
                Box::new(move |result| {
                    let Ok(entry) = result else {
                        return WalkState::Continue;
                    };

                    let is_allowed = entry
                        .path()
                        .extension()
                        .and_then(|e| e.to_str())
                        .is_some_and(|ext| opts.allowed_exts.iter().any(|a| a == ext));

                    if is_allowed {
                        visitor.visit(f(&entry));
                    }
                    return WalkState::Continue;
                })
            });
    }
}
