pub mod login;
pub mod logout;
use axum::Router;

pub fn auth_factory(app: Router) -> Router {
    app
}
