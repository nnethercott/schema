/// TSG helpers for python classes
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

    ;; top-level file name
    attr (@class.node) filename = global_filename
}
"#
        };
    }

    #[macro_export]
    macro_rules! wrapped_classes {
        () => {
            format!(
                r#"
;; wrapped classes
(decorated_definition
    {}
    (class_definition) @class
)
{{
    attr (@class.node) decorator = (source-text @decorator_name)
}}
"#,
                $crate::decorator!()
            )
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
    attr (@fn.node) common_attrs = @fn
    attr (@fn.node) name = (source-text @fn_name)

    ;; edge annotations
    edge @class.node -> @fn.node
    edge @fn.node -> @class.node
    attr (@class.node -> @fn.node) kind = "method"
    attr (@fn.node -> @class.node) kind = "_parent"
}
"#
        };
    }

    #[macro_export]
    macro_rules! methods_params {
        () => {
            r#"
;; methods
(class_definition
    body: (block
        (function_definition
            parameters: ((_) @params (#not-eq? @params "()"))
        ) @fn
    )
)
{
    attr (@fn.node) params = (source-text @params)
}
"#
        };
    }

    #[macro_export]
    macro_rules! methods_returns {
        () => {
            r#"
;; methods
(class_definition
    body: (block
        (function_definition
            return_type: (_)? @returns
        ) @fn
    )
)
{
    if some @returns{
        attr (@fn.node) returns = (source-text @returns)
    }
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
    edge @fn.node -> @class.node
    attr (@class.node -> @fn.node) kind = "method"
    attr (@fn.node -> @class.node) kind = "_parent"
}}
"#,
                $crate::decorator!()
            )
        };
    }

    #[macro_export]
    macro_rules! wrapped_methods_params {
        () => {
            format!(
                r#"
;; methods
(class_definition
    body: (block
        (decorated_definition
            {}
            (function_definition
                parameters: ((_) @params (#not-eq? @params "()"))
            ) @fn
        )
    )
)
{{
    attr (@fn.node) params = (source-text @params)

    ;; hack: all captures must be used
    let _ = @decorator_name
}}
"#,
                $crate::decorator!()
            )
        };
    }

    #[macro_export]
    macro_rules! wrapped_methods_returns {
        () => {
            format!(
                r#"
;; methods
(class_definition
    body: (block
        (decorated_definition
            {}
            (function_definition
                return_type: (_)? @returns
            ) @fn
        )
    )
)
{{
    if some @returns{{
        attr (@fn.node) returns = (source-text @returns)
    }}

    ;; hack: all captures must be used
    let _ = @decorator_name
}}
"#,
                $crate::decorator!()
            )
        };
    }

    #[macro_export]
    macro_rules! methods_calls {
        () => {
            format!(
                r#"
;; method calls
(class_definition
    body: (block
        (function_definition
            {}
        ) @fn
    )
)
{{
    node @call.node
    attr (@call.node) common_attrs = @call
    attr (@call.node) name = (source-text @call_name)

    ;; edge annotations
    edge @fn.node -> @call.node
    edge @call.node -> @fn.node
    attr (@fn.node -> @call.node) kind = "call"
    attr (@call.node -> @fn.node) kind = "_parent"
}}
"#,
                $crate::_body_calls!()
            )
        };
    }

    #[macro_export]
    macro_rules! wrapped_methods_calls {
        () => {
            format!(
                r#"
;; wrapped method calls
(class_definition
    body: (block
        (decorated_definition
            {}
            definition: (
                function_definition
                    {}
            ) @fn
        )
    )
)
{{
    node @call.node
    attr (@call.node) common_attrs = @call
    attr (@call.node) name = (source-text @call_name)

    ;; edge annotations
    edge @fn.node -> @call.node
    edge @call.node -> @fn.node
    attr (@fn.node -> @call.node) kind = "call"
    attr (@call.node -> @fn.node) kind = "_parent"

    ;; hack: all captures must be used
    let _ = @decorator_name
}}
"#,
                $crate::decorator!(),
                $crate::_body_calls!()
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
                    {}{}{}{}{}{}{}{}{}{}{}
                "#,
                $crate::common_attributes!(),
                $crate::classes!(),
                $crate::wrapped_classes!(),
                $crate::methods!(),
                $crate::methods_params!(),
                $crate::methods_returns!(),
                $crate::methods_calls!(),
                $crate::wrapped_methods!(),
                $crate::wrapped_methods_params!(),
                $crate::wrapped_methods_returns!(),
                $crate::wrapped_methods_calls!(),
            )
        };
    }
}
