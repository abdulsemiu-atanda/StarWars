mod api;

use api::api_tools::*;
use api::models::*;

fn main() {
    let endpoint = api_url("people");
    let response = api_response(&endpoint);
    let data: ApiResponse = response.json().unwrap();

    println!("Hello, world!");

    for item in &data.results {
        println!("{}", item.summarize());
    }
}
