use draveur::{
    Result, class_stanzas, draveur::Draveur, functions_stanzas, lang::Python, query_decorated_classes, query_decorated_functions,
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
    let functions = String::from("((module)) @all");

    Draveur::<Python>::new()
        .add(functions, functions_stanzas!())?
        .add(classes, class_stanzas!())?
        .waltz("./")?;
        // .waltz("/Users/naten/mistral/dashboard/workflow_sdk/")?;

    println!("{:?}", now.elapsed());
    Ok(())
}
