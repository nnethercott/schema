/// TS helpers for common captures
mod queries {
    /// Generic macro for querying decorated definitions
    #[macro_export]
    macro_rules! _query_decorated {
        // query_decorated!(class_definition)
        ($def_type:ident) => {
            format!(
                r#"
(decorated_definition
    ({})
    definition: ({})
) @body
            "#,
                $crate::decorator!(),
                stringify!($def_type)
            )
        };

        // query_decorated!(class_definition, "foo", "bar")
        ($def_type:ident, $($d:literal),+ $(,)?) => {{
            let allowlist = [$($d),+].join(" ");
            format!(
                r#"
(decorated_definition
    ({})
    definition: ({})
    (#any-of? @decorator_name {})
) @body
            "#,
                $crate::decorator!(),
                stringify!($def_type),
                allowlist
            )
        }};
    }

    #[macro_export]
    macro_rules! query_decorated_classes {
        () => { $crate::_query_decorated!(class_definition) };
        ($($d:literal),+ $(,)?) => { $crate::_query_decorated!(class_definition, $($d),+) };
    }

    #[macro_export]
    macro_rules! query_decorated_functions {
        () => { $crate::_query_decorated!(function_definition) };
        ($($d:literal),+ $(,)?) => { $crate::_query_decorated!(function_definition, $($d),+) };
    }

    /// General decorated objects capture
    #[macro_export]
    macro_rules! query_decorated_objects {
        // dec!()
        () => {
            format!("(decorated_definition ({})) @body", $crate::_decorator!())
        };

        // dec!("foo", "bar")
        ($($d:literal),+ $(,)?) => {{
            let allowlist = [$($d),+].join(" ");
            format!(
                "(decorated_definition ({}) (#any-of? @decorator_name {})) @body",
                $crate::decorator!(),
                allowlist
            )
        }};
    }

    #[macro_export]
    macro_rules! query_functions {
        () => {
           "(module (function_definition) @fn)"
        };
    }

    #[macro_export]
    macro_rules! query_classes {
        () => {
           "(class_definition) @class" 
        };
    }
}
