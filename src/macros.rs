// TODO: play around with query structure, some syntaxes _seem_ faster
// maybe deeper patterns take longer to match, should move fast cases first,
// or check code statistically to see distribution

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
