/// TSG helpers for python functions
mod functions {
    #[macro_export]
    macro_rules! functions {
        () => {
            r#"
(function_definition
  name: (identifier) @fn_name
) @fn
{
    node @fn.node
    attr (@fn.node) common_attrs = @fn
    attr (@fn.node) name = (source-text @fn_name)
}
"#
        };
    }

    #[macro_export]
    macro_rules! wrapped_functions {
        () => {
            format!(
                r#"

(decorated_definition
    {}
    (function_definition) @fn
)
{{
    attr (@fn.node) decorator = (source-text @decorator_name)
}}
"#,
                $crate::decorator!()
            )
        };
    }

    #[macro_export]
    macro_rules! params {
        () => {
            r#"
(function_definition
  parameters: ((_) @params (#not-eq? @params "()"))
) @fn
{
    attr (@fn.node) params = (source-text @params)
}
"#
        };
    }

    #[macro_export]
    macro_rules! returns {
        () => {
            r#"
(function_definition
  return_type: (_)? @returns
) @fn
{
    if some @returns{
        attr (@fn.node) returns = (source-text @returns)
    }
}
"#
        };
    }

    #[macro_export]
    macro_rules! functions_stanzas {
        // stanzas!() - all stanzas
        () => {
            format!(
                r#"
                    global global_filename
                    global global_row
                    global global_column
                    {}{}{}{}{}
                "#,
                $crate::common_attributes!(),
                $crate::functions!(),
                $crate::wrapped_functions!(),
                $crate::params!(),
                $crate::returns!(),
            )
        };
    }
}

mod expressions {
    #[macro_export]
    macro_rules! call {
        () => {};
    }

    #[macro_export]
    macro_rules! assignment {
        () => {};
    }
}
