use draveur::{crawl::{CrawlOpts, Crawler, DoNothingVisitor}, draveur::tree_sitter_parse};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let opts = CrawlOpts::default()
        .path("/Users/naten/mistral/dashboard/")
        .threads(10)
        .add_ext("py");

    let crawler = Crawler::new(opts);
    crawler.crawl(tree_sitter_parse, DoNothingVisitor);

    println!("{:?}", now.elapsed());
}
