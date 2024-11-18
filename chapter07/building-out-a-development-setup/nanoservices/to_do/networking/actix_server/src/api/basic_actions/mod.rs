pub mod create;
pub mod get;
pub mod delete;
pub mod update;

use actix_web::web::{ServiceConfig, get, scope, post, delete, patch};


/// Registers the basic actions routes so we can mount them in the server.
/// 
/// # Arguments
/// - `app` - a mutable reference to a `ServiceConfig` instance.
pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
        .route("get/all", get().to(get::get_all))
        .route("get/{name}", get().to(get::get_by_name))
        .route("create", post().to(create::create))
        .route("delete/{name}", delete().to(delete::delete_by_name))
        .route("update", patch().to(update::update))
    );
}
