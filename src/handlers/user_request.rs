use crate::api::api_tools::*;
use crate::api::models::*;

pub fn people(url: Option<String>) -> ApiResponse<Person> {
  let endpoint = url.unwrap_or(api_url("people"));
  let response = api_response(&endpoint);
  let data: ApiResponse<Person> = response.json().unwrap();

  data
}

pub fn starships(url: Option<String>) -> ApiResponse<Starship> {
  let endpoint = url.unwrap_or(api_url("starships"));
  let response = api_response(&endpoint);
  let data: ApiResponse<Starship> = response.json().unwrap();

  data
}

pub fn films(url: Option<String>) -> ApiResponse<Film> {
  let endpoint = url.unwrap_or(api_url("films"));
  let response = api_response(&endpoint);
  let data: ApiResponse<Film> = response.json().unwrap();

  data
}

pub fn resource_summaries(input: String, url: Option<String>) -> StarWarsItem {
  let trimmed_input = input.trim();

  match trimmed_input {
    "S" => {
      let starships = starships(url);

      StarWarsItem {
        count: starships.count,
        next: starships.next,
        previous: starships.previous,
        results: starships.results.iter().map(|starship| starship.summarize()).collect::<Vec<_>>()
      }
    },
    "F" => {
      let films = films(url);

      StarWarsItem {
        count: films.count,
        next: films.next,
        previous: films.previous,
        results: films.results.iter().map(|film| film.summarize()).collect::<Vec<_>>()
      }
    },
    "P" => {
      let people = people(url);

      StarWarsItem {
        count: people.count,
        next: people.next,
        previous: people.previous,
        results: people.results.iter().map(|person| person.summarize()).collect::<Vec<_>>()
      }
    },
    _ => {
      let people = people(url);

      StarWarsItem {
        count: people.count,
        next: people.next,
        previous: people.previous,
        results: people.results.iter().map(|person| person.summarize()).collect::<Vec<_>>()
      }
    }
  }
}
