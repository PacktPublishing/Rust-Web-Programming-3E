pub mod create;
pub mod get;
pub mod delete;
pub mod update;

use actix_web::web::{ServiceConfig, get, scope, post, delete, put};
use dal::to_do_items::descriptors::SqlxPostGresDescriptor;
use auth_kernel::user_session::descriptors::RedisSessionDescriptor;


pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
        .route("get/all", get().to(
            get::get_all::<
                SqlxPostGresDescriptor,
                RedisSessionDescriptor
            >)
        )
        .route("create", post().to(
            create::create::<
                SqlxPostGresDescriptor,
                RedisSessionDescriptor
            >)
        )
        .route("delete/{name}", delete().to(
            delete::delete_by_name::<
                SqlxPostGresDescriptor,
                RedisSessionDescriptor
            >)
        )
        .route("update", put().to(
            update::update::<
                SqlxPostGresDescriptor,
                RedisSessionDescriptor
            >)
        )
    );
}
