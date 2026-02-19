use std::fmt::Debug;
use std::io;
use std::path::Path;
use thiserror::Error;
use tree_sitter::{LanguageError, QueryError};
use tree_sitter_graph::ParseError;

use crate::lang::Lang;

pub type Result<T> = std::result::Result<T, crate::errors::Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] IoErrorKind),

    #[error("language")]
    Lang {
        language: String,
        #[source]
        source: LanguageError,
    },

    #[error(transparent)]
    TreeSitter(#[from] TreeSitterError),

    #[error(transparent)]
    Crawl(#[from] ignore::Error),

    #[error("failed to parse tree")]
    Parse,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn lang<L: Lang>(source: LanguageError) -> Self {
        Self::Lang {
            source,
            language: L::NAME.into(),
        }
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum IoErrorKind {
    #[error("failed to open {file}")]
    Open {
        file: String,
        #[source]
        source: io::Error,
    },
    #[error("failed to read {file}")]
    Read {
        file: String,
        #[source]
        source: io::Error,
    },
    #[error("failed to mmap {file}")]
    Mmap {
        file: String,
        #[source]
        source: io::Error,
    },
}

impl IoErrorKind {
    pub fn read(file: impl AsRef<Path>, source: io::Error) -> Self {
        Self::Read {
            file: file.as_ref().display().to_string(),
            source,
        }
    }
    pub fn open(file: impl AsRef<Path>, source: io::Error) -> Self {
        Self::Open {
            file: file.as_ref().display().to_string(),
            source,
        }
    }
    pub fn mmap(file: impl AsRef<Path>, source: io::Error) -> Self {
        Self::Mmap {
            file: file.as_ref().display().to_string(),
            source,
        }
    }
}

#[derive(Error, Debug)]
pub enum TreeSitterError {
    #[error("invalid query: {0}\n{1}")]
    Query(QueryError, String),
    #[error("invalid stanzas: {0}\n{1}")]
    Stanzas(ParseError, String),
}
