pub mod login;
pub mod logout;

use actix_web::web::{ServiceConfig, get, scope};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;


pub fn auth_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1/auth")
        .route("login", get().to(
            login::login::<SqlxPostGresDescriptor>)
        )
    );
}
