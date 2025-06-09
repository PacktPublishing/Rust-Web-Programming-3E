pub mod login;
pub mod logout;

use rocket::Route;


pub fn auth_factory() -> Vec<Route> {
    routes![
        login::login
    ]
}
