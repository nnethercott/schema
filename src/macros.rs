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

// FIXME: capture function parameter types

mod stanzas {
    #[macro_export]
    macro_rules! stanza_classes {
        () => {
        r#"
;; classes
(class_definition
    name: (identifier) @class_name
) @class
{
    node @class.node
    attr (@class.node) name = (source-text @class_name)
    attr (@class.node) kind = "class"
}
"#
        };
    }

    #[macro_export]
    macro_rules! stanza_methods {
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
    attr (@fn.node) src = (source-text @fn)

    ;; edge annotations
    edge @class.node -> @fn.node
    attr (@class.node -> @fn.node) rel = "method"
}
"#
        };
    }

    #[macro_export]
    macro_rules! stanza_wrapped_methods {
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
    attr (@fn.node) wrapper = (source-text @decorator_name)
    attr (@fn.node) name = (source-text @fn_name)
    attr (@fn.node) src = (source-text @fn)

    ;; edge annotations
    edge @class.node -> @fn.node
    attr (@class.node -> @fn.node) rel = "wrapped_method"
}}
"#,
                $crate::decorator!()
            )
        };
    }
}

#[macro_export]
macro_rules! stanzas {
    // stanzas!() - all stanzas
    () => {
        format!(
            "{}{}{}",
            $crate::stanza_classes!(),
            $crate::stanza_methods!(),
            $crate::stanza_wrapped_methods!()
        )
    };
}
