use draveur::{
    crawl::{CrawlOpts, Crawler, DoNothingVisitor},
    decorated_objects,
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
        // .path("/Users/naten/mistral/dashboard/workflow_sdk/")
        .threads(10)
        .add_ext("py");

    let crawler = Crawler::new(opts);

    // these just go inside of the draveur...
    let query = build_query(&decorated_objects!(
        "workflows.workflow.define",
        "workflows.update",
        "workflows.query",
        "workflows.signal",
        "activity",
        "foo"
    ));
    // let query = build_query("(module)@all");
    let stanzas =
        ast::File::from_str(tree_sitter_python::LANGUAGE.into(), &stanzas!()).expect(&stanzas!());
    println!("{}", &stanzas!());

    let tls = ThreadLocal::with_capacity(10);

    crawler.crawl(
        |e| tree_sitter_parse(e, &query, &stanzas, &tls),
        DoNothingVisitor,
    );

    println!("{:?}", now.elapsed());
}
