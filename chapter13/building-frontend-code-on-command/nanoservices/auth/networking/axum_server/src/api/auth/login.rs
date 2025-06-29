use auth_dal::users::transactions::get::GetByEmail;
use auth_core::api::auth::login::login as core_login;
use glue::errors::NanoServiceError;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::extract_auth::extract_credentials;   // ↙ same helper you built earlier
use crate::extract_auth::Credentials;           // (optional) re-export for clarity

/// POST /login
///
/// Expects an `Authorization: Basic …` header containing `email:password`.
/// On success returns `200 OK` with the generated token in the body.
pub async fn login<T: GetByEmail>(
    req: Request<Body>,
) -> Result<impl IntoResponse, NanoServiceError> {
    // 1. Pull Basic-Auth credentials from the header
    let creds = extract_credentials(&req)?;

    // 2. Run core business logic (may hit the database through T)
    let token = core_login::<T>(creds.email, creds.password).await?;

    // 3. Emit JSON { token: … } (whatever `core_login` returns)
    Ok((StatusCode::OK, Json(token)))
}
