mod calls {
    #[macro_export]
    macro_rules! _sync_calls {
        () => {
            r#"(call [function: (identifier) function: (attribute (_) .)] @call_name) @call"#
        };
    }

    #[macro_export]
    macro_rules! _async_calls {
        () => {
            format!("(await {})", $crate::_sync_calls!())
        };
    }

    #[macro_export]
    macro_rules! _calls {
        () => {
            format!("[{} {}]", $crate::_sync_calls!(), $crate::_async_calls!())
        };
    }

    /// Expression containing a call (direct or assigned)
    #[macro_export]
    macro_rules! _call_expr {
        () => {
            format!(
                r#"(expression_statement [{} {} (assignment left: (_) right: [{} {}])])"#,
                $crate::_sync_calls!(),
                $crate::_async_calls!(),
                $crate::_sync_calls!(),
                $crate::_async_calls!()
            )
        };
    }

    #[macro_export]
    macro_rules! _with_block {
        () => {
            format!("(with_statement body: (block {}))", $crate::_call_expr!())
        };
    }

    /// Body block pattern for capturing function calls within a block
    /// Handles direct calls and calls inside with statements
    #[macro_export]
    macro_rules! _body_calls {
        () => {
            format!("body: (block [{} {}])", $crate::_call_expr!(), $crate::_with_block!())
        };
    }
}
