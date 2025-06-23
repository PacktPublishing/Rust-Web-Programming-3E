pub mod login;
pub mod logout;

use rocket::Route;
use rocket::routes;


pub fn auth_factory() -> Vec<Route> {
    routes![
        login::login,
    ]
}
