// src/api/mod.rs
pub mod auth;
pub mod users;
use axum::Router;


pub fn views_factory(app: Router) -> Router {
    let app = users::users_factory(app);
    let app = auth::auth_factory(app);
    app
}
