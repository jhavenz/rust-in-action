use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}


pub fn run () {
    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    let as_json = json!(&calabar);

    println!("City Details: {:#}", as_json);
}
