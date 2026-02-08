use std::time::Instant;
use schema::{s_expr, parse::{build_py_query}};
use schema::parse::Noeud;
use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    let lang = tree_sitter_python::LANGUAGE.into();
    parser.set_language(&lang).unwrap();

    let source_code = r#"
def foo(arg: Any):
    def decorator(f: Callable):
        def _decorator(*args, **kwargs):
            print(arg)
            print(f(*args, **kwargs))
        return _decorator

    return decorator

@foo(42)
def nate():
    print("hi")

@foo
def jack():
    pass
    "#;

    let tree = parser.parse(source_code, None).unwrap();
    let root = Noeud::new(tree.root_node(), source_code.as_bytes());

    let now = Instant::now();
    let query = build_py_query(&s_expr!("foo","bar"));
    let parsed: Vec<_> = root.parse(&query).collect();
    println!("{:?}", now.elapsed());

    for item in parsed {
        dbg!(item);
    }
}
