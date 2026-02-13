#[macro_export]
macro_rules! dec_s_expr {
    ($($d:literal),*) => {
        {
            let mut v = vec![];
            $(
                v.push(format!("{}", $d));
            )*
            let allowlist = v.join(" ");
            let query = format!(r#"
 (decorated_definition
    (decorator[
        ;; @foo(*args)
        ;; @foo.bar.baz(*args)
        (call
            [
                function: (identifier) @decorator.name
                function: (attribute (_) .) @decorator.name
            ]
        )
        ;; @foo
        ;; @foo.bar.baz
        [
            (identifier) @decorator.name
            (attribute (_) .) @decorator.name
        ]
    ])(#any-of? @decorator.name {}))@body
                        "#,
                allowlist);
            query
        }
    };
}

// * assumes ctx is parsed from s-expr earlier
// - one stanza for class nodes
// - one stanza for fn nodes
//  - make general stanza for all captured functions to add labels
//  - make another one like here specialize to class methods?: https://docs.rs/tree-sitter-graph/latest/tree_sitter_graph/reference/index.html#graph-nodes
// FIXME: bit of work to be done on parsing function parameter types

#[macro_export]
macro_rules! stanzas {
    () => {{
        format!(
            r#"
        ;; classes
        (class_definition
            name: (identifier) @class_name) @class
        {{
            node @class.node
            attr (@class.node) name = (source-text @class_name)
            attr (@class.node) kind = "class"
        }}

        ;; function definitions
        (function_definition
          name: (identifier) @fn_name) @fn
        {{
            node @fn.node
            attr (@fn.node) name = (source-text @fn_name)
            attr (@fn.node) kind = "fn"
        }}

        ;; class methods
        ;; (class_definition
        ;;     name: (identifier) @_class_name
        ;;     body: (block
        ;;         (function_definition
        ;;         name: (identifier)) @fn
        ;;     )
        ;; ) @_class
        ;; {{
        ;;     ;; figure stuff out
        ;; }}

    "#
        )
    }};
}
