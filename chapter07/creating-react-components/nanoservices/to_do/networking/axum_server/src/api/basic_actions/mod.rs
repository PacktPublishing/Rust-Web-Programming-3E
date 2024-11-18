pub mod create;
pub mod get;
pub mod delete;
pub mod update;

use axum::{
    routing::get,
    routing::post,
    routing::delete,
    routing::patch,
    Router,
};


/// Registers the basic actions routes so we can mount them in the server.
///
/// # Arguments
/// - `router` - a mutable reference to a `Router` instance.
pub fn basic_actions_factory(router: Router) -> Router {
    router
        .route("/api/v1/get/all", get(get::get_all))
        .route("/api/v1/get/:name", get(get::get_by_name))
        .route("/api/v1/create", post(create::create))
        .route("/api/v1/delete/:name", delete(delete::delete_by_name))
        .route("/api/v1/update", patch(update::update))
}
