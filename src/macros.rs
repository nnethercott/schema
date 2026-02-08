// FIXME: use fuzzy matching with a `#match?` statement ?
#[macro_export]
macro_rules! s_expr {
    ($($d:literal),*) => {
        {
            let mut v = vec![];
            $(
                v.push(format!("{}", $d));
            )*
            let allowlist = v.join(" ");
            let query = format!(r#"
                (decorated_definition
                (decorator
                    (call
                    function: (identifier) @decorator.name
                    arguments: (argument_list) @decorator.args)*
                    (identifier)*  @decorator.ident )
                    (_)@body
                    (#any-of? @decorator.name {}))
                        "#, 
                allowlist);

            query
        }
    };
}
