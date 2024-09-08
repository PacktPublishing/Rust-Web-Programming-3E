#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::status::Custom;
use std::net::Ipv4Addr;

/// A basic handler that receives a request and returns a greeting.
/// 
/// # Arguments
/// * `name` - The name extracted from the request path
/// 
/// # Returns
/// A string response
#[get("/<name>")]
async fn greet(name: Option<String>) -> Custom<String> {
    let name = name.unwrap_or_else(|| "World".to_string());
    Custom(Status::Ok, format!("Hello {}!", name))
}


/// A basic handler that returns a greeting with no inputs.
/// 
/// # Returns
/// A string response
#[get("/")]
async fn greet_root() -> Custom<String> {
    greet(Some("World".to_string())).await
}


/// A basic handler that returns a greeting with no inputs.
/// 
/// # Returns
/// A string response
#[get("/say/hello")]
async fn say_hello() -> &'static str {
    "Hello Again!"
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = rocket::Config {
        port: 8080,  // Set the desired port number here
        address: Ipv4Addr::new(127, 0, 0, 1).into(),
        ..Default::default()
    };

    let _rocket = rocket::custom(&config)
        .mount("/", routes![greet_root, greet, say_hello])
        .launch()
        .await?;

    Ok(())
}
