use std::io;
use std::path::Path;

// syntax-related enum(query, tsg)
// ts parsing error
// anyhow transparent
use thiserror::Error;
use tree_sitter::LanguageError;

pub type Result<T> = std::result::Result<T, crate::errors::Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] IoErrorKind),

    // FIXME: make into a nice struct with the associated `impl Lang` ?
    #[error("language")]
    Lang(#[from] LanguageError),

    #[error(transparent)]
    Crawl(#[from] ignore::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
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
