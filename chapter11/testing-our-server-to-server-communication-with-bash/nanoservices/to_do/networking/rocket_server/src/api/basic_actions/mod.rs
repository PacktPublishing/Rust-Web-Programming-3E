pub mod create;
pub mod get;
pub mod delete;
pub mod update;

use rocket::Route;


/// Registers the basic actions routes so we can mount them in the server.
/// 
/// # Returns
/// A vector of `Route`s.
pub fn basic_actions_factory() -> Vec<Route> {
    routes![
        get::get_all, 
        create::create, 
        delete::delete_by_name,
        update::update
    ]
}
