use auth_dal::users::transactions::get::GetByEmail;
use auth_core::api::auth::login::login as core_login;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use auth_kernel::user_session::transactions::login::LoginUserSession;

use crate::extract_auth::extract_credentials;   // ↙ same helper you built earlier
use crate::extract_auth::Credentials;           // (optional) re-export for clarity

/// POST /login
///
/// Expects an `Authorization: Basic …` header containing `email:password`.
/// On success returns `200 OK` with the generated token in the body.
pub async fn login<T: GetByEmail, X: LoginUserSession>(
    req: Request<Body>,
) -> Result<impl IntoResponse, NanoServiceError> {
    // 1. Pull Basic-Auth credentials from the header
    let credentials = extract_credentials(&req)?;

    // 2. Run core business logic (may hit the database through T)
    let token = core_login::<T>(credentials.email.clone(), credentials.password).await?;
    let user = T::get_by_email(credentials.email).await?;
    let url = std::env::var("CACHE_API_URL").map_err(|e|{
        NanoServiceError::new(
            e.to_string(), 
            NanoServiceErrorStatus::Unknown
        )
    })?;
    let _ = X::login_user_session(
        &url,
        &user.unique_id,
        20,
        user.id
    ).await?;

    // 3. Emit JSON { token: … } (whatever `core_login` returns)
    Ok((StatusCode::OK, Json(token)))
}
