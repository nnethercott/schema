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
