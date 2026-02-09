use ignore::DirEntry;
use schema::crawl::{CrawlOpts, Crawler};
use schema::parse::Noeud;
use schema::{parse::build_query, dec_s_expr};
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use tree_sitter::Parser;

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
        dbg!(entry);
    }
}
