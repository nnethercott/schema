use tree_sitter::Query;
use tree_sitter_graph::ast::File;

#[cfg(feature = "python")]
pub use python::*;

use crate::{Result, TreeSitterError};

pub trait Lang {
    // Associated file extension for language
    const NAME: &'static str;
    const EXT: &'static str;

    // NOTE: may need an associated type who we can deserialize serde_json::Values into

    //ts language
    fn language() -> tree_sitter::Language;

    // default implementations
    fn build_query(s_expr: String) -> Result<Query> {
        let lang = Self::language();
        let query = Query::new(&lang, &s_expr).map_err(|_| TreeSitterError::Query(s_expr))?;
        Ok(query)
    }

    fn build_stanzas(stanzas: String) -> Result<File> {
        let lang = Self::language();
        let stanzas =
            File::from_str(lang, &stanzas).map_err(|_| TreeSitterError::Stanzas(stanzas))?;
        Ok(stanzas)
    }
}

#[cfg(feature = "python")]
pub mod python {
    use super::Lang;
    use tree_sitter_python;

    pub struct Python;

    impl Lang for Python {
        const NAME: &'static str = "python";
        const EXT: &'static str = "py";

        fn language() -> tree_sitter::Language {
            tree_sitter_python::LANGUAGE.into()
        }
    }
}
