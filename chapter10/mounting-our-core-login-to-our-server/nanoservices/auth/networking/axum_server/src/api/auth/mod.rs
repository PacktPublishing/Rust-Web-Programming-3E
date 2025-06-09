pub mod login;
pub mod logout;
use axum::{
    routing::get,
    routing::post,
    routing::delete,
    routing::patch,
    Router,
};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;

pub fn auth_factory(app: Router) -> Router {
    app.route(
        "/api/v1/auth/login",
        post(login::login::<SqlxPostGresDescriptor>),
    )
}
