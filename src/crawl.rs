//! Traverses file directory ignoring globbed patterns in .gitignore
//! use ignore Walker with configurable threads

use ignore::{DirEntry, WalkBuilder, WalkState};
use std::path::PathBuf;
use std::sync::Arc;

pub struct CrawlOpts {
    pub path: PathBuf,
    pub allowed_exts: Vec<String>,
}

impl Default for CrawlOpts {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./"),
            allowed_exts: vec!["py".into()],
        }
    }
}

pub struct Crawler {
    opts: Arc<CrawlOpts>,
}

impl Crawler {
    pub fn new(opts: CrawlOpts) -> Self {
        Self {
            opts: Arc::new(opts),
        }
    }

    pub fn crawl<F>(&self, f: F)
    where
        F: Fn(&DirEntry) + Send + Sync,
    {
        let opts = Arc::clone(&self.opts);
        let f = Arc::new(f);

        WalkBuilder::new(&self.opts.path)
            .follow_links(false)
            .standard_filters(true)
            .build_parallel()
            .run(|| {
                let opts = Arc::clone(&opts);
                let f = Arc::clone(&f);
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
                        f(&entry);
                    }
                    return WalkState::Continue;
                })
            });
    }
}
