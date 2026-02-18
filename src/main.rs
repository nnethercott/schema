use draveur::{
    Result, class_stanzas, decorated_objects, draveur::Draveur, functions_stanzas, lang::Python,
};
use std::time::Instant;

fn main() -> Result<()> {
    let now = Instant::now();

    let classes = decorated_objects!(
        // "workflows.workflow.define",
        // "workflows.update",
        // "workflows.query",
        // "workflows.signal",
        // "activity",
        "foo"
    );
    let functions = String::from("((module)) @all");

    // let path = "./";
    let path = "/Users/naten/mistral/dashboard/workflow_sdk/";

    Draveur::<Python>::new()
        .add(functions.clone(), functions_stanzas!())?
        // .add(functions, class_stanzas!())?
        .waltz("./")?;
        // .waltz("/Users/naten/mistral/dashboard/workflow_sdk/")?;

    println!("{:?}", now.elapsed());
    Ok(())
}
