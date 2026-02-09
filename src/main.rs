use ignore::DirEntry;
use schema::crawl::{CrawlOpts, Crawler};
use schema::parse::Noeud;
use schema::{dec_s_expr, parse::build_query};
use tree_sitter_graph::functions::Functions;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use tree_sitter::Parser;
use tree_sitter_graph::{ExecutionConfig, NoCancellation, Variables, ast, functions};

fn main() {
    let now = Instant::now();

    let mut opts = CrawlOpts::default();
    opts.path = "/Users/naten/mistral/dashboard/workflow_sdk/".into();
    let crawler = Crawler::new(opts);
    crawler.crawl(tree_sitter_parse);

    println!("{:?}", now.elapsed());
}

/// Applies to each file in the glob **/*.py
/// builds AST and extracts decorator definitions
fn tree_sitter_parse(e: &DirEntry) {
    let mut f = File::open(e.path()).unwrap();
    let mut parser = Parser::new();
    let lang = tree_sitter_python::LANGUAGE.into();
    parser.set_language(&lang).unwrap();

    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();
    let tree = parser.parse(&buf, None).unwrap();
    let root = Noeud::new(tree.root_node(), &buf);

    let query = build_query(&dec_s_expr!("workflows.workflow.define","activity"));
    let mut hits = root.parse(&query);

    while let Some(entry) = hits.next() {
        for e in entry{
            // dbg!(&e);
            let (_, sample) = &e;
            tree_sitter_graph(sample, &mut parser);
        }
    }
}

fn tree_sitter_graph(node: &Noeud, parser: &mut Parser) {
    // https://github.com/tree-sitter/tree-sitter-graph/blob/main/tests/it/parser.rs
    let source = r#"
        (function_definition
          name: (identifier) @_cap1) @cap2
        {
          node loc1
          node @cap2.prop1
          edge @cap2.prop1 -> loc1
          attr (@cap2.prop1 -> loc1) precedence
          attr (@cap2.prop1) push = "str2", pop
          var @cap2.var1 = loc1
          set @cap2.var1 = loc1
        }
    "#;

    let file =
        ast::File::from_str(tree_sitter_python::LANGUAGE.into(), source).expect("Cannot parse file");

    let tree = parser.parse(&node.ctx, None).unwrap();
    // https://github.com/tree-sitter/tree-sitter-graph/blob/main/tests/it/execution.rs
    let functions = Functions::stdlib();
    let globals = Variables::new();
    let mut config = ExecutionConfig::new(&functions, &globals);
    let graph = file.execute(&tree, node.ctx_as_str(), &mut config, &NoCancellation).unwrap();

    println!("{}", graph.pretty_print());
}
