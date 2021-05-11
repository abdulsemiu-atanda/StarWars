use console::style;
use loading::Loading;
use std::collections::HashMap;

mod api;
mod handlers;

use handlers::user_request::*;
use handlers::terminal::*;
use api::models::*;

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

    let data: StarWarsItem = resource_summaries(input, None);

    finish_loader_and_display_result(&mut loading, &data);

    if data.next.is_some() {
        let next = display_dialog(&["Yes", "No", "Suprise Me"], "Please select one of these options to load more");

        if next == "Yes" || next == "Suprise Me" {
            let data: StarWarsItem = resource_summaries(String::from(hash.get(&data.item_type).unwrap()), data.next);

            finish_loader_and_display_result(&mut loading, &data);
        }
    }

    loading.end();
}
