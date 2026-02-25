use draveur_python::{
    Python, class_stanzas, functions_stanzas, query_decorated_classes, query_functions,
};

use draveur::{Result, draveur::Draveur};
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
        .waltz("/Users/naten/mistral/dashboard/workflow_sdk/")?;
        // .waltz("/Users/naten/coding/rust/draveur/")?;

    for item in &graphs{
        for node in item.iter(){
            let n = node.clone();
            n.is_leaf();
        }
    }

    let elapsed = now.elapsed();

    // for graph in graphs {
    //     println!("{}", serde_json::to_string_pretty(&graph)?);
    // }

    println!("{:?}", elapsed);
    Ok(())
}
