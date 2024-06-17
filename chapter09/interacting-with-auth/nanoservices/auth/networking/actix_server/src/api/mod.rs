pub mod auth;
pub mod users;

use actix_web::web::ServiceConfig;


pub fn views_factory(app: &mut ServiceConfig) {
    users::users_factory(app);
    auth::auth_factory(app);
}
