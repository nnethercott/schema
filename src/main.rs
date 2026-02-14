use draveur::draveur::Draveur;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let draveur = Draveur::new();
    // draveur.waltz("/Users/naten/mistral/dashboard/workflow_sdk/");
    draveur.waltz("./");

    println!("{:?}", now.elapsed());
}
