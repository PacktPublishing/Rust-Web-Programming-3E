pub mod create;
pub mod get;
pub mod delete;
pub mod update;

use axum::{
    routing::get,
    Router,
};


/// Registers the basic actions routes so we can mount them in the server.
///
/// # Arguments
/// - `router` - a mutable reference to a `Router` instance.
pub fn basic_actions_factory(router: Router) -> Router {
    router
        .route("/api/v1/get/all", get(get::get_all))
}
