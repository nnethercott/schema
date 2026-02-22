use draveur::{
    Result, class_stanzas, draveur::Draveur, functions_stanzas, lang::Python,
    query_decorated_classes, query_functions,
};
use std::time::Instant;

fn main() -> Result<()> {
    let now = Instant::now();

    let classes = query_decorated_classes!(
        "workflows.workflow.define",
        "workflows.update",
        "workflows.query",
        "workflows.signal",
        "workflows.activity",
        "foo"
    );
    let functions = query_functions!().to_string();

    let graphs = Draveur::<Python>::new()
        .add(functions, functions_stanzas!())?
        .add(classes, class_stanzas!())?
        .waltz("./")?;
        // .waltz("/Users/naten/mistral/dashboard/workflow_sdk/")?;

    let elapsed = now.elapsed();

    for graph in graphs {
        println!("{}", serde_json::to_string_pretty(&graph)?);
    }

    println!("{:?}", elapsed);
    Ok(())
}
