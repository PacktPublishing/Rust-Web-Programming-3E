pub mod create;

use auth_dal::users::descriptors::SqlxPostGresDescriptor;
use actix_web::web::{ServiceConfig, scope, post};


pub fn users_factory(app: &mut ServiceConfig) {
}
