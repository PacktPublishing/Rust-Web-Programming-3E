pub mod create;
pub mod get;
pub mod delete;
pub mod update;

use actix_web::web::{ServiceConfig, get, scope};


pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
        .route("get/all", get().to(get::get_all))
        .route("get/{name}", get().to(get::get_by_name))
    );
}
