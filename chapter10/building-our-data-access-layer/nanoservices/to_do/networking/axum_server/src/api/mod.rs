pub mod basic_actions;
use axum::Router;


/// Registers all the views for the API.
///
/// # Arguments
/// - `router` - a mutable reference to a `Router` instance.
/// 
/// # Returns
/// A `Router` instance with all the views registered.
pub fn views_factory(router: Router) -> Router {
    let router = basic_actions::basic_actions_factory(router);
    router
}
