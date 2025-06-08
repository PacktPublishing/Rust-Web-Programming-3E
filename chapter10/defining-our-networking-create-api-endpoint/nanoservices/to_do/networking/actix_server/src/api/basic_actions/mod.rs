pub mod create;
pub mod get;
pub mod delete;
pub mod update;
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;
use actix_web::web::{ServiceConfig, get, scope, post, delete, put};


/// Registers the basic actions routes so we can mount them in the server.
/// 
/// # Arguments
/// - `app` - a mutable reference to a `ServiceConfig` instance.
pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
        .route("get/all", get().to(
            get::get_all::<SqlxPostGresDescriptor>)
        )
        .route("create", post().to(
            create::create::<SqlxPostGresDescriptor>)
        )
        .route("delete/{name}", delete().to(
            delete::delete_by_name::<SqlxPostGresDescriptor>)
        )
        .route("update", put().to(
            update::update::<SqlxPostGresDescriptor>)
        )
    );
}

