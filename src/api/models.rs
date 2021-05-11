use serde_derive::{Serialize, Deserialize};
use chrono::{NaiveDate};
use chrono::prelude::*;

#[derive(Serialize, Deserialize,Debug)]
pub struct Person {
  pub name: String,
  pub height: String,
  pub hair_color: String,
  pub skin_color: String,
  pub eye_color: String,
  pub birth_year: String,
  pub gender: String,
  pub homeworld: String,
  pub films: Vec<String>,
  pub species: Vec<String>,
  pub vehicles: Vec<String>,
  pub starships: Vec<String>,
  pub created: String,
  pub edited: String,
  pub url: String,
}

pub trait Summary {
  fn summarize(&self) -> String;
}

impl Summary for Person {
  fn summarize(&self) -> String {
    format!("I am {}, {} inches tall born in {} with {} eyes and {} skin", self.name, self.height, self.birth_year, self.eye_color, self.skin_color)
  }
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Film {
  pub title: String,
  pub episode_id: u32,
  pub opening_crawl: String,
  pub director: String,
  pub producer: String,
  pub release_date: String,
  pub characters: Vec<String>,
  pub planets: Vec<String>,
  pub starships: Vec<String>,
  pub vehicles: Vec<String>,
  pub species: Vec<String>,
  pub created: String,
  pub edited: String,
  pub url: String,
}

impl Summary for Film {
  fn summarize(&self) -> String {
    let date = NaiveDate::parse_from_str(&self.release_date[..], "%Y-%m-%d").unwrap();
    let formatted_date = Utc.ymd(date.year(), date.month(), date.day());

    format!(
      "{} was released on {}, directed by {} and produced by {}",
      self.title,
      formatted_date.format("%a %b %e %Y").to_string(),
      self.director,
      self.producer
    )
  }
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Starship {
  pub name: String,
  pub model: Option<String>,
  pub manufacturer: String,
  pub cost_in_credits: String,
  pub length: String,
  pub max_atmosphering_speed: String,
  pub crew: String,
  pub passengers: String,
  pub cargo_capacity: String,
  pub consumables: String,
  pub hyperdrive_rating: String,
  #[serde(rename = "MGLT", default)]
  pub mglt: String,
  pub starship_class: String,
  pub pilots: Vec<String>,
  pub films: Vec<String>,
  pub created: String,
  pub edited: String,
  pub url: String,
}

impl Summary for Starship {
  fn summarize(&self) -> String {
    format!("Starship {} was maunfactured by {} costing {} with a maximum speed of {} and a cargo capacity of {}.\
     It can hold {} crew members and can transport up to {} passengers",
     self.name,
     self.manufacturer,
     self.cost_in_credits,
     self.max_atmosphering_speed,
     self.cargo_capacity,
     self.crew,
     self.passengers
    )
  }
}

#[derive(Serialize, Deserialize,Debug)]
pub struct ApiResponse<T: Summary> {
  pub count: u32,
  pub next: Option<String>,
  pub previous: Option<String>,
  pub results: Vec<T>
}

pub struct StarWarsItem {
  pub count: u32,
  pub next: Option<String>,
  pub previous: Option<String>,
  pub results: Vec<String>
}
