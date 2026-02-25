use tree_sitter::Query;
use tree_sitter_graph::ast::File;

use crate::{Result, TreeSitterError};

pub trait Lang {
    const NAME: &'static str;
    const EXT: &'static str;

    fn language() -> tree_sitter::Language;

    fn build_query(s_expr: String) -> Result<Query> {
        let lang = Self::language();
        let query = Query::new(&lang, &s_expr).map_err(|e| TreeSitterError::Query(e, s_expr))?;

        Ok(query)
    }

    fn build_stanzas(stanzas: String) -> Result<File> {
        let lang = Self::language();
        let stanzas =
            File::from_str(lang, &stanzas).map_err(|e| TreeSitterError::Stanzas(e, stanzas))?;
        Ok(stanzas)
    }
}
