mod calls;
mod classes;
mod control;
mod decorators;
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
    start_col = (plus global_column (start-column node)),
    start_row = (plus global_row (start-row node)),
    end_col = (plus global_column (end-column node)),
    end_row = (plus global_row (end-row node))
            "#
        };
    }
}
