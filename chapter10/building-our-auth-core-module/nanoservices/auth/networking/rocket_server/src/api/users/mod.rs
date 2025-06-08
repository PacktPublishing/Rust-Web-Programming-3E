pub mod create;
use axum::Router;

pub fn users_factory(app: Router) -> Router {
    app
}
