//! The entry point for the Rocket server.
#[macro_use] extern crate rocket;
use std::net::Ipv4Addr;
use auth_dal::migrations::run_migrations;

mod api;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    run_migrations().await;
    let config = rocket::Config {
        port: 8081,
        address: Ipv4Addr::new(127, 0, 0, 1).into(),
        ..Default::default()
    };

    let rocket = rocket::custom(&config);
    let rocket = rocket.mount("/api/v1", api::auth::auth_factory());
    let rocket = rocket.mount("/api/v1", api::users::users_factory());
    rocket.launch().await?;
    Ok(())
}
