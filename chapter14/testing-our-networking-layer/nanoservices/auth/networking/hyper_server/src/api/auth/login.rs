use hyper::{
    body::{Bytes, Incoming},
    header,
    Request, Response,
};
use http_body_util::Full;
use serde_json::to_vec;

use glue::{
    errors::{NanoServiceError, NanoServiceErrorStatus},
    safe_eject,
};

use auth_core::api::auth::login::login as core_login;
use auth_dal::users::transactions::get::GetByEmail;

use crate::extract_auth::extract_credentials; // assumes you re-exported the helper

/// POST /api/v1/auth/login
///
/// Reads `Authorization: Basic …`, validates the user via `core_login`,
/// and returns the issued token as JSON.
pub async fn login<T: GetByEmail>(
    req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    // ── 1. Parse credentials from the header (borrows the request) ──────────
    let creds = extract_credentials(&req)?;

    // ── 2. Business logic / database hit via the generic DAL impl ───────────
    let token = core_login::<T>(creds.email, creds.password).await?;

    // ── 3. Serialize the token and build a JSON response ────────────────────
    let json_body = to_vec(&token).map_err(|e| NanoServiceError::new(
        e.to_string(),
        NanoServiceErrorStatus::Unknown,
    ))?;

    safe_eject!(
        Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .status(200)
            .body(Full::new(Bytes::from(json_body))),
        NanoServiceErrorStatus::Unknown
    )
}
