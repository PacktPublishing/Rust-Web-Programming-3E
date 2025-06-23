use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
    serde::json::Json,
};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;
use auth_core::api::auth::login::login as core_login;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use rocket::serde::Serialize;
use rocket::get;

use crate::extract_auth::{extract_credentials, Credentials};
use auth_kernel::user_session::transactions::login::LoginUserSession;
use auth_kernel::user_session::descriptors::RedisSessionDescriptor;
use auth_dal::users::transactions::get::GetByEmail;


#[rocket::async_trait]
impl<'r> FromRequest<'r> for Credentials {
    type Error = NanoServiceError;               // the error you already have

    async fn from_request(req: &'r Request<'_>)
        -> request::Outcome<Self, Self::Error>
    {
        match extract_credentials(req) {          // call your helper
            Ok(creds) => Outcome::Success(creds), // happy path
            Err(e) => Outcome::Error((          // map to a failure
                Status::Unauthorized,
                e,
            )),
        }
    }
}

/// POST /auth/login
///
/// Expects an `Authorization: Basic <base64(email:password)>` header.
/// On success returns `{ token: … }` (whatever `core_login` emits).
#[get("/login")]
pub async fn login(creds: Credentials) -> Result<Json<impl Serialize>, NanoServiceError> {
    // 1. Parse `email` / `password` from the header.
    let Credentials { email, password } = creds;

    // 2. Call domain logic (backs onto the DAL via SqlxPostGresDescriptor).
    let token = core_login::<SqlxPostGresDescriptor>(email.clone(), password).await?;

    let user = SqlxPostGresDescriptor::get_by_email(email).await?;
    let url = std::env::var("CACHE_API_URL").map_err(|e|{
        NanoServiceError::new(
            e.to_string(), 
            NanoServiceErrorStatus::Unknown
        )
    })?;
    let _ = RedisSessionDescriptor::login_user_session(
        &url,
        &user.unique_id,
        20,
        user.id
    ).await?;

    // 3. Return JSON with default 200 OK.
    Ok(Json(token))
}
