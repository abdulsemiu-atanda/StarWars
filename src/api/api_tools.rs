pub fn api_url(path: &str) -> String {
  format!("https://swapi.dev/api/{}", path)
}

pub fn api_response(url: &String) -> reqwest::blocking::Response {
  reqwest::blocking::get(url).unwrap()
}
