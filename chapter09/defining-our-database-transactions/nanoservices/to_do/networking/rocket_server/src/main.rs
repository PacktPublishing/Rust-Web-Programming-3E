//! The entry point for the Rocket server.
#[macro_use] extern crate rocket;
use std::net::Ipv4Addr;

mod api;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = rocket::Config {
        port: 8080,  // Set the desired port number here
        address: Ipv4Addr::new(127, 0, 0, 1).into(),
        ..Default::default()
    };

    let rocket = rocket::custom(&config);
    let rocket = rocket.mount("/api/v1", api::basic_actions::basic_actions_factory());
    rocket.launch().await?;
    Ok(())
}
