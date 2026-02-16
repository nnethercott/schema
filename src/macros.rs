mod decorators {
    /// Extracts name from decorated block
    #[macro_export]
    macro_rules! decorator {
        () => {
            r#"(decorator
                [
                    ;; @a()
                    ;; @a.b()
                    (call
                        [
                            function: (identifier) @decorator_name
                            function: (attribute (_) .) @decorator_name
                        ]
                    )
                    ;; @a
                    ;; @a.b
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

// TODO: capture function parameter types
// TODO: common attributes like line number and such

mod common {
    #[macro_export]
    macro_rules! common_attributes {
        () => {
            r#"
attribute common_attrs = node => 
    ;; src = (source-text node), 
    type = (node-type node),

    ;; NOTE: offsets start at first item in the capture -> unintuitive
    ;; start_col = (plus global_column (start-column node)), 
    start_row = (plus global_row (start-row node)),
    ;; end_col = (plus global_column (end-column node)), 
    end_row = (plus global_row (end-row node))
            "#
        };
    }
}

mod classes {
    #[macro_export]
    macro_rules! classes {
        () => {
            r#"
;; classes
(class_definition
    name: (identifier) @class_name
) @class
{
    node @class.node
    attr (@class.node) name = (source-text @class_name)
    attr (@class.node) common_attrs = @class
    attr (@class.node) filename = global_filename
}
"#
        };
    }

    #[macro_export]
    macro_rules! methods {
        () => {
            r#"
;; methods
(class_definition
    body: (block
        (function_definition
            name: (identifier) @fn_name
        ) @fn
    )
) @class
{
    node @fn.node
    attr (@fn.node) name = (source-text @fn_name)
    attr (@fn.node) common_attrs = @fn

    ;; edge annotations
    edge @class.node -> @fn.node
    attr (@class.node -> @fn.node) rel = "method"
}
"#
        };
    }

    #[macro_export]
    macro_rules! wrapped_methods {
        () => {
            format!(
                r#"
;; wrapped methods
(class_definition
    body: (block
        (decorated_definition
            {}
            definition: (
                function_definition
                    name: (identifier) @fn_name
            ) @fn
        ) 
    )
) @class
{{
    node @fn.node
    attr (@fn.node) decorator = (source-text @decorator_name)
    attr (@fn.node) name = (source-text @fn_name)
    attr (@fn.node) common_attrs = @fn

    ;; edge annotations
    edge @class.node -> @fn.node
    attr (@class.node -> @fn.node) rel = "method"
}}
"#,
                $crate::decorator!()
            )
        };
    }

    #[macro_export]
    macro_rules! class_stanzas {
        // stanzas!() - all stanzas
        () => {
            format!(
                r#"
                    global global_filename 
                    global global_row 
                    global global_column
                    {}{}{}{}
                "#,
                $crate::common_attributes!(),
                $crate::classes!(),
                $crate::methods!(),
                $crate::wrapped_methods!(),
            )
        };
    }
}

mod functions {
    #[macro_export]
    macro_rules! functions {
        () => {
            r#"
(module 
    (function_definition
        name: (identifier) @fn_name
    ) @fn
)
{
    node @fn.node
    attr (@fn.node) name = (source-text @fn_name)
    attr (@fn.node) common_attrs = @fn
}
"#
        };
    }

    #[macro_export]
    macro_rules! functions_stanzas  {
        // stanzas!() - all stanzas
        () => {
            format!(
                r#"
                    global global_filename 
                    global global_row 
                    global global_column
                    {}{}
                "#,
                $crate::common_attributes!(),
                $crate::functions!(),
            )
        };
    }
}

