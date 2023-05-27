use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Configuration {
    port: u16,
    api_url: String,
}

fn main() {
    let c = envy::from_env::<Configuration>().expect("Please provide required env variables");

    println!("{:#?}", c);
}
