use draveur::{
    crawl::{CrawlOpts, Crawler},
    decorated_objects,
    draveur::Draveur,
    lang::{Python},
    stanzas,
};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    // let query = "(module)@all";
    let _query = decorated_objects!(
        "workflows.workflow.define",
        "workflows.update",
        "workflows.query",
        "workflows.signal",
        "activity",
        "foo"
    );

    let opts = CrawlOpts::default()
        // .path("/Users/naten/mistral/dashboard/workflow_sdk/")
        .path("./")
        .threads(10)
        .add_lang::<Python>();
    let crawler = Crawler::new(opts);

    Draveur::<Python>::new(stanzas!()).waltz(crawler);

    println!("{:?}", now.elapsed());
}
