/// TSG helpers for conditional blocks (if-else-elif)
mod conditionals {
    /// Captures if_statement as a node and links to parent function
    #[macro_export]
    macro_rules! if_elif_else {
        () => {
            r#"
(function_definition
  body: (block
    (if_statement) @if_stmt
  )
) @fn
{
    node @if_stmt.node
    attr (@if_stmt.node) common_attrs = @if_stmt
    attr (@if_stmt.node) kind = "if_statement"

    edge @fn.node -> @if_stmt.node
    edge @if_stmt.node -> @fn.node
    attr (@fn.node -> @if_stmt.node) kind = "if-else"
    attr (@if_stmt.node -> @fn.node) kind = "_parent"
}
"#
        };
    }

    /// Captures calls within the main if block (consequence)
    #[macro_export]
    macro_rules! if_arm {
        () => {
                r#"
(function_definition
  body: (block
    (if_statement
        condition: (_) @cond
        consequence: (_) @if_arm
    ) @if_stmt
  )
)
{
    ;; link stmt to arm
    node @if_arm.node
    attr (@if_arm.node) which = "if"
    attr (@if_arm.node) common_attrs = @if_arm
    attr (@if_arm.node) condition = (source-text @cond)

    edge @if_stmt.node -> @if_arm.node
    edge @if_arm.node -> @if_stmt.node
    attr (@if_stmt.node -> @if_arm.node) kind = "branch"
    attr (@if_arm.node -> @if_stmt.node) kind = "_parent"
}
"#
        };
    }
    /// Captures calls within the main if block (consequence)
    #[macro_export]
    macro_rules! if_arm_calls {
        () => {
            format!(
                r#"
(function_definition
  body: (block
    (if_statement
        consequence: (block [{} {}]) @if_arm
    )
  )
)
{{
    ;; link calls to arm
    node @call.node
    attr (@call.node) common_attrs = @call
    attr (@call.node) name = (source-text @call_name)
    attr (@call.node) branch = "if"

    edge @if_arm.node -> @call.node
    edge @call.node -> @if_arm.node
    attr (@if_arm.node -> @call.node) kind = "branch"
    attr (@call.node -> @if_arm.node) kind = "_parent"
}}
"#,
                $crate::_call_expr!(),
                $crate::_with_block!()
            )
        };
    }

    /// Captures elif condition and links arm to if_stmt
    #[macro_export]
    macro_rules! elif_arm {
        () => {
            r#"
(function_definition
  body: (block
    (if_statement
      alternative: (elif_clause
        condition: (_) @cond
        consequence: (_) @elif_arm
      )
    ) @if_stmt
  )
)
{
    ;; link stmt to arm
    node @elif_arm.node
    attr (@elif_arm.node) which = "elif"
    attr (@elif_arm.node) common_attrs = @elif_arm
    attr (@elif_arm.node) condition = (source-text @cond)

    edge @if_stmt.node -> @elif_arm.node
    edge @elif_arm.node -> @if_stmt.node
    attr (@if_stmt.node -> @elif_arm.node) kind = "branch"
    attr (@elif_arm.node -> @if_stmt.node) kind = "_parent"
}
"#
        };
    }

    /// Captures calls within elif blocks, links to arm
    #[macro_export]
    macro_rules! elif_arm_calls {
        () => {
            format!(
                r#"
(function_definition
  body: (block
    (if_statement
      alternative: (elif_clause
        consequence: (block [{} {}]) @elif_arm
      )
    )
  )
)
{{
    ;; link calls to arm
    node @call.node
    attr (@call.node) common_attrs = @call
    attr (@call.node) name = (source-text @call_name)

    edge @elif_arm.node -> @call.node
    edge @call.node -> @elif_arm.node
    attr (@elif_arm.node -> @call.node) kind = "call"
    attr (@call.node -> @elif_arm.node) kind = "_parent"
}}
"#,
                $crate::_call_expr!(),
                $crate::_with_block!()
            )
        };
    }

    /// Captures else arm and links to if_stmt (no condition for else)
    #[macro_export]
    macro_rules! else_arm {
        () => {
            r#"
(function_definition
  body: (block
    (if_statement
      alternative: (else_clause
        body: (_) @else_arm
      )
    ) @if_stmt
  )
)
{
    ;; link stmt to arm
    node @else_arm.node
    attr (@else_arm.node) which = "else"
    attr (@else_arm.node) common_attrs = @else_arm

    edge @if_stmt.node -> @else_arm.node
    edge @else_arm.node -> @if_stmt.node
    attr (@if_stmt.node -> @else_arm.node) kind = "branch"
    attr (@else_arm.node -> @if_stmt.node) kind = "_parent"
}
"#
        };
    }

    /// Captures calls within else blocks, links to arm
    #[macro_export]
    macro_rules! else_arm_calls {
        () => {
            format!(
                r#"
(function_definition
  body: (block
    (if_statement
      alternative: (else_clause
        body: (block [{} {}]) @else_arm
      )
    )
  )
)
{{
    ;; link calls to arm
    node @call.node
    attr (@call.node) common_attrs = @call
    attr (@call.node) name = (source-text @call_name)
    attr (@call.node) branch = "else"

    edge @else_arm.node -> @call.node
    edge @call.node -> @else_arm.node
    attr (@else_arm.node -> @call.node) kind = "call"
    attr (@call.node -> @else_arm.node) kind = "_parent"
}}
"#,
                $crate::_call_expr!(),
                $crate::_with_block!()
            )
        };
    }

    /// Combined stanza for all if-else-elif patterns
    #[macro_export]
    macro_rules! conditionals {
        () => {
            format!(
                "{}{}{}{}",
                $crate::if_elif_else!(),
                $crate::if_arm!(),
                // $crate::if_arm_calls!(),
                $crate::elif_arm!(),
                // $crate::elif_arm_calls!(),
                $crate::else_arm!(),
                // $crate::else_arm_calls!(),
            )
        };
    }
}
