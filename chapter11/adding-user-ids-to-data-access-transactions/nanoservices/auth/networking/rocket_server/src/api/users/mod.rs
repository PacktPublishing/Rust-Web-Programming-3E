pub mod create;
pub mod get;

use rocket::Route;
use rocket::routes;


pub fn users_factory() -> Vec<Route> {
    routes![
        create::create,
        get::get
    ]
}
