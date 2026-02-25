use draveur::Lang;

pub mod macros;

#[cfg(feature = "bindings")]
pub mod bindings;

// language definition
pub struct Python;

impl Lang for Python {
    const NAME: &'static str = "python";
    const EXT: &'static str = "py";

    fn language() -> tree_sitter::Language {
        tree_sitter_python::LANGUAGE.into()
    }
}
