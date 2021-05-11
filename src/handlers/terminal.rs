use dialoguer::{theme::ColorfulTheme, Select};
use loading::Loading;
use std::thread;
use std::time::Duration;

use crate::api::models::*;

pub fn display_dialog(selections: &[&'static str; 3]) -> String {
  let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select one of these options to get started")
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

pub fn finish_loader_and_display_result(loading: &mut Loading, data: StarWarsItem) {
  loading.success("OK");

  println!();
  println!("{}", "-".repeat(120));
  loading.end();

  for item in data.results {
      println!("{}", item);
  }

  println!("{}", "-".repeat(120));
}
