use crate::api::api_tools::*;
use crate::api::models::*;

pub fn people() -> ApiResponse<Person> {
  let endpoint = api_url("people");
  let response = api_response(&endpoint);
  let data: ApiResponse<Person> = response.json().unwrap();

  data
}

pub fn starships() -> ApiResponse<Starship> {
  let endpoint = api_url("starships");
  let response = api_response(&endpoint);
  let data: ApiResponse<Starship> = response.json().unwrap();

  data
}

pub fn films() -> ApiResponse<Film> {
  let endpoint = api_url("films");
  let response = api_response(&endpoint);
  let data: ApiResponse<Film> = response.json().unwrap();

  data
}

pub fn resource_summaries(input: &String) -> StarWarsItem {
  let trimmed_input = input.trim();

  match trimmed_input {
    "S" => {
      let starships = starships();

      StarWarsItem {
        count: starships.count,
        next: starships.next,
        previous: starships.previous,
        results: starships.results.iter().map(|starship| starship.summarize()).collect::<Vec<_>>()
      }
    },
    "F" => {
      let films = films();

      StarWarsItem {
        count: films.count,
        next: films.next,
        previous: films.previous,
        results: films.results.iter().map(|film| film.summarize()).collect::<Vec<_>>()
      }
    },
    "P" => {
      let people = people();

      StarWarsItem {
        count: people.count,
        next: people.next,
        previous: people.previous,
        results: people.results.iter().map(|person| person.summarize()).collect::<Vec<_>>()
      }
    },
    _ => {
      let people = people();

      StarWarsItem {
        count: people.count,
        next: people.next,
        previous: people.previous,
        results: people.results.iter().map(|person| person.summarize()).collect::<Vec<_>>()
      }
    }
  }
}
