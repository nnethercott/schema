use draveur::{decorated_objects, draveur::Draveur, parse::build_query, stanzas};
use tree_sitter_graph::ast;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    // let query = build_query("(module)@all");
    let query = build_query(decorated_objects!(
        "workflows.workflow.define",
        "workflows.update",
        "workflows.query",
        "workflows.signal",
        "activity",
        "foo"
    ));
    let stanzas = ast::File::from_str(tree_sitter_python::LANGUAGE.into(), &stanzas!())
        .expect(&stanzas!());

    let draveur = Draveur::new(query, stanzas);
    // draveur.waltz("/Users/naten/mistral/dashboard/workflow_sdk/");
    draveur.waltz("./");

    println!("{:?}", now.elapsed());
}
