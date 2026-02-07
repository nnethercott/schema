use std::time::Instant;

use schema::parse::{self, NodeLike};
use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    let lang = tree_sitter_python::LANGUAGE.into();
    parser.set_language(&lang).unwrap();

    let source_code = r#"
from collections.abc import Callable
from typing import Any

def foo(arg: Any):
    def decorator(f: Callable):
        def _decorator(*args, **kwargs):
            print(arg)
            print(f(*args, **kwargs))
        return _decorator

    return decorator

def bar():
    pass

@foo(42)
def nate():
    print("hi")
    "#;

    let tree = parser.parse(source_code, None).unwrap();
    let root = NodeLike::new(tree.root_node(), source_code.as_bytes());
    // let text_provider = root_node.to_sexp();

//     let query = r#"
// (function_definition
//   (identifier) @top
//   (#any-of? @top
//    "foo"
//    "bar"
//    "decorator"))@body
//     "#;

    let query = r#"
(function_definition (_))@nate
    "#;

    let now = Instant::now();
    let parsed = parse::parse(root, query);
    println!("{:?}", now.elapsed());

    for node in parsed{
        println!("{}", node);
    }

}
