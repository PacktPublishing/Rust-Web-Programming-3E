pub mod create;
use actix_web::web::{ServiceConfig, scope, post};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;


pub fn users_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1/users")
        .route("create", post().to(
            create::create::<SqlxPostGresDescriptor>)
        )
    );
}
