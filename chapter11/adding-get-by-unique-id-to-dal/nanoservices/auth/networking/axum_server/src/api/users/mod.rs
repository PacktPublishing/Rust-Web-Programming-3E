pub mod create;
use axum::{
    routing::get,
    routing::post,
    routing::delete,
    routing::patch,
    Router,
};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;


pub fn users_factory(app: Router) -> Router {
    app.route("/api/v1/users/create", post(create::create::<SqlxPostGresDescriptor>))
}
