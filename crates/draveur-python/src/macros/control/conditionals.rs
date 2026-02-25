mod conditionals {
    #[macro_export]
    macro_rules! if_nodes {
        () => {
            r#"
(if_statement
    condition: (_) @cond
) @if

{
    node @if.node
    attr (@if.node) kind = "conditional"
    attr (@if.node) common_attrs = @if
    attr (@if.node) condition = (source-text @cond)
}
"#
        };
    }

    #[macro_export]
    macro_rules! fn_to_if {
        () => {
            r#"
(function_definition
    body: (block
        (if_statement) @if
    )
) @fn

{
    edge @fn.node -> @if.node
    attr (@fn.node -> @if.node) kind = "entry"
}
"#
        };
    }

    #[macro_export]
    macro_rules! if_edge {
        () => {
            r#"
(if_statement
    consequence: (block
        (if_statement) @child
    )
) @parent

{
    edge @parent.node -> @child.node
    attr (@parent.node -> @child.node) kind = "if"
}
"#
        };
    }

    #[macro_export]
    macro_rules! elif_edge {
        () => {
            r#"
(if_statement
    alternative: (elif_clause
        condition: (_) @cond
    ) @elif
) @parent

{
    node @elif.node
    attr (@elif.node) kind = "conditional"
    attr (@elif.node) common_attrs = @elif
    attr (@elif.node) condition = (source-text @cond)

    edge @parent.node -> @elif.node
    attr (@parent.node -> @elif.node) kind = "elif"
}
"#
        };
    }

    #[macro_export]
    macro_rules! else_edge {
        () => {
            r#"
(if_statement
    alternative: (else_clause) @else
) @parent

{
    node @else.node
    attr (@else.node) kind = "conditional"
    attr (@else.node) common_attrs = @else

    edge @parent.node -> @else.node
    attr (@parent.node -> @else.node) kind = "else"
}
"#
        };
    }
}

#[macro_export]
macro_rules! conditionals {
    () => {
        format!(
            "{}{}{}{}{}",
            $crate::if_nodes!(),
            $crate::fn_to_if!(),
            $crate::if_edge!(),
            $crate::elif_edge!(),
            $crate::else_edge!(),
        )
    };
}
