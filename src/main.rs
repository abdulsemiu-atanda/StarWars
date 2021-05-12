use console::style;
use loading::Loading;
use std::collections::HashMap;

mod api;
mod handlers;

use handlers::terminal::*;

fn main() {
    let mut hash = HashMap::new();

    // setup hash to match to shorthand values
    hash.insert("People".to_string(), "P".to_string());
    hash.insert("Starships".to_string(), "S".to_string());
    hash.insert("Films".to_string(), "F".to_string());

    let selections = &["People", "Starships", "Films"];

    println!("{}", style("Welcome Star Wars!").cyan());

    let response = display_dialog(&selections, "Please select one of these options to get started");
    let input = String::from(hash.get(&response).unwrap());

    // setup loader
    let mut loading = Loading::new();

    setup_loader(&mut loading);

    user_action(input, None, &mut loading, hash);

    loading.end();
}
