use tree_sitter::Query;
use tree_sitter_graph::ast::File;

#[cfg(feature = "python")]
pub use python::*;

pub trait Lang {
    // Associated file extension for language
    const EXT: &'static str;

    // NOTE: may need an associated type who we can deserialize serde_json::Values into

    //ts language
    fn language() -> tree_sitter::Language;

    // default implementations
    fn build_query(s_expr: String) -> Query {
        let lang = Self::language();
        Query::new(&lang, &s_expr).expect(&format!("invalid ts query:\n{}", s_expr))
    }

    fn build_stanzas(stanzas: String) -> File {
        let lang = Self::language();
        File::from_str(lang, &stanzas).expect(&format!("invalid tsg:\n{}", &stanzas))
    }
}

#[cfg(feature = "python")]
pub mod python {
    use super::Lang;
    use tree_sitter_python;

    pub struct Python;

    impl Lang for Python {
        const EXT: &'static str = "py";

        fn language() -> tree_sitter::Language {
            tree_sitter_python::LANGUAGE.into()
        }
    }
}
