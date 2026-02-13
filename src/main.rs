use draveur::{
    crawl::{CrawlOpts, Crawler, DoNothingVisitor},
    dec_s_expr,
    draveur::tree_sitter_parse,
    parse::build_query,
    stanzas,
};
use std::time::Instant;
use thread_local::ThreadLocal;
use tree_sitter_graph::ast;

fn main() {
    let now = Instant::now();

    let opts = CrawlOpts::default()
        .path("/Users/naten/mistral/dashboard/")
        .threads(10)
        .add_ext("py");

    let crawler = Crawler::new(opts);

    let tls = ThreadLocal::with_capacity(10);
    let query = build_query(&dec_s_expr!("workflows.workflow.define", "activity", "foo"));
    let stanzas = ast::File::from_str(tree_sitter_python::LANGUAGE.into(), &stanzas!()).unwrap();

    crawler.crawl(
        |e| tree_sitter_parse(e, &query, &stanzas, &tls),
        DoNothingVisitor,
    );

    println!("{:?}", now.elapsed());
}
