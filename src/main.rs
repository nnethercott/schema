use draveur::{decorated_objects, draveur::Draveur, lang::Python, stanzas};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let query = decorated_objects!(
        "workflows.workflow.define",
        "workflows.update",
        "workflows.query",
        "workflows.signal",
        "activity",
        "foo"
    );
    // let query = String::from("(module)@all");

    let path = "./";
    // let path = "/Users/naten/mistral/dashboard/workflow_sdk/";
    Draveur::<Python>::new_with_candidates(stanzas!(), query)
        .waltz(path)
        .unwrap();
    // Draveur::<Python>::new(stanzas!()).waltz(path).unwrap();

    println!("{:?}", now.elapsed());
}
