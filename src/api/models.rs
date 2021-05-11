use serde_derive::{Serialize, Deserialize};

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
pub struct ApiResponse {
  pub count: u32,
  pub next: Option<String>,
  pub previous: Option<String>,
  pub results: Vec<Person>
}
