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

pub fn resource_summaries(input: &String) -> Vec<String> {
  let trimmed_input = input.trim();

  match trimmed_input {
    "S" => {
      let starships = starships();

      starships.results.iter().map(|starship| starship.summarize()).collect::<Vec<_>>()
    },
    "F" => {
      let films = films();

      films.results.iter().map(|film| film.summarize()).collect::<Vec<_>>()
    },
    "P" => {
      let people = people();

      people.results.iter().map(|person| person.summarize()).collect::<Vec<_>>()
    },
    _ => {
      let people = people();

      people.results.iter().map(|person| person.summarize()).collect::<Vec<_>>()
    }
  }
}
