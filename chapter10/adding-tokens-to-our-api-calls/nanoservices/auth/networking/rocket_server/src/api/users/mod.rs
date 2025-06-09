pub mod create;

use rocket::Route;
use rocket::routes;


pub fn users_factory() -> Vec<Route> {
    routes![
        create::create
    ]
}
