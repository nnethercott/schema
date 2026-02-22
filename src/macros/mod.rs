mod calls;
mod classes;
mod conditionals;
mod functions;
mod queries;

mod common {
    #[macro_export]
    macro_rules! common_attributes {
        () => {
            r#"
attribute common_attrs = node =>
    src = (source-text node),
    type = (node-type node),

    ;; NOTE: offsets start at first item in the capture -> unintuitive
    ;;start_col = (plus global_column (start-column node)),
    start_row = (plus global_row (start-row node)),
    ;; end_col = (plus global_column (end-column node)),
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
}
