pub mod basic_actions;
use actix_web::web::ServiceConfig;


/// Registers all the views for the API.
/// 
/// 
/// # Arguments
/// - `app` - a mutable reference to a `ServiceConfig` instance.
pub fn views_factory(app: &mut ServiceConfig) {
    basic_actions::basic_actions_factory(app);
}
