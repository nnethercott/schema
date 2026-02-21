mod classes;
mod functions;
mod expr;

mod common {
    #[macro_export]
    macro_rules! common_attributes {
        () => {
            r#"
attribute common_attrs = node =>
    ;; src = (source-text node),
    type = (node-type node),

    ;; NOTE: offsets start at first item in the capture -> unintuitive
    start_col = (plus global_column (start-column node)),
    start_row = (plus global_row (start-row node)),
    end_col = (plus global_column (end-column node)),
    end_row = (plus global_row (end-row node))
            "#
        };
    }
}

mod decorators {
    /// Extracts name from decorated block
    #[macro_export]
    macro_rules! decorator {
        () => {
            r#"(decorator
                [
                    ;; @a() and @a.b()
                    (call
                        [
                            function: (identifier) @decorator_name
                            function: (attribute (_) .) @decorator_name
                        ]
                    )
                    ;; @a and @a.b
                    [
                        (identifier) @decorator_name
                        (attribute (_) .) @decorator_name
                    ]
                ]
            )
            ;; only capture last decorator if multiple
            @_ ."#
        };
    }

    /// Captures decorated objects
    #[macro_export]
    macro_rules! decorated_objects {
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
}

mod decorated_queries {
    /// Generic macro for querying decorated definitions
    #[macro_export]
    macro_rules! query_decorated {
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
        () => { $crate::query_decorated!(class_definition) };
        ($($d:literal),+ $(,)?) => { $crate::query_decorated!(class_definition, $($d),+) };
    }

    #[macro_export]
    macro_rules! query_decorated_functions {
        () => { $crate::query_decorated!(function_definition) };
        ($($d:literal),+ $(,)?) => { $crate::query_decorated!(function_definition, $($d),+) };
    }
}
