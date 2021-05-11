use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use loading::Loading;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

mod api;
mod handlers;

use handlers::user_request::*;
use api::models::*;

fn main() {
    let mut hash = HashMap::new();

    // setup hash to match to shorthand values
    hash.insert("People".to_string(), "P".to_string());
    hash.insert("Starships".to_string(), "S".to_string());
    hash.insert("Films".to_string(), "F".to_string());

    let selections = &["People", "Starships", "Films"];

    println!("{}", style("Welcome Star Wars!").cyan());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select one of these options to get started")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let response = String::from(selections[selection]);
    let input = hash.get(&response).unwrap();

    // setup loader
    let mut loading = Loading::new();

    loading.start();

    for i in 0..20 {
        let string = if i % 2 == 0 { String::from("..") } else { String::from("...") };

        loading.text(format!("Loading{}", string));
        thread::sleep(Duration::from_millis(50));
    }

    let data: StarWarsItem = resource_summaries(&input);

    loading.success("OK");
    println!();
    println!("{}", "-".repeat(120));
    loading.end();

    for item in &data.results {
        println!("{}", item);
    }
    println!("{}", "-".repeat(120));
}
