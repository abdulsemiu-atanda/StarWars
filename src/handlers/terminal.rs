use dialoguer::{theme::ColorfulTheme, Select};
use loading::Loading;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use rand::Rng;

use crate::api::models::*;
use crate::handlers::user_request::*;

pub fn display_dialog(selections: &[&'static str; 3], prompt: &str) -> String {
  let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let response = String::from(selections[selection]);

    response
}

pub fn setup_loader(loading: &mut Loading) {
  loading.start();

  for i in 0..20 {
      let string = if i % 2 == 0 { String::from("..") } else { String::from("...") };

      loading.text(format!("Loading{}", string));
      thread::sleep(Duration::from_millis(50));
  }
}

pub fn finish_loader_and_display_result(loading: &mut Loading, data: &StarWarsItem) {
  loading.success("OK");

  println!();
  println!("{}", "-".repeat(120));

  for item in &data.results {
      println!("{}", item);
  }

  println!("{}", "-".repeat(120));
}

pub fn action(input: &str) -> &str {
  let mut randomizer = rand::thread_rng();
  let options = vec!("Yes", "No");
  let index = randomizer.gen_range(0..2);

  if input == "Suprise Me" {
    options[index]
  } else { input }
}

pub fn user_action(input: String, url: Option<String>, loading: &mut Loading, type_shorthand_map: HashMap<String, String>) {
  let data: StarWarsItem = resource_summaries(input, url);

  if data.next.is_some() {
    finish_loader_and_display_result(loading, &data); // display the result of the latest call before asking to select more options

    let next = display_dialog(&["Yes", "No", "Suprise Me"], "Please select one of these options to load more");
    let action = action(&next[..]);

    if action == "Yes" {
      let input_map = type_shorthand_map.clone();
      let next_input = type_shorthand_map.get(&data.item_type).unwrap().to_string();

      user_action(next_input, data.next, loading, input_map)
    } else { return }
  } else {
    finish_loader_and_display_result(loading, &data)
  }
}
