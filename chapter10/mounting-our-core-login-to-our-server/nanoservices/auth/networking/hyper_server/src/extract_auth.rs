use hyper::{header, Request, body::Incoming};
use base64::{engine::general_purpose, Engine};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

/// Extracts `Authorization: Basic …` credentials from a Hyper request.
///
/// *Takes only an immutable reference, so the caller can still move the
/// request into `extract_body` or any other consumer afterwards.*
pub fn extract_credentials(
    req: &Request<Incoming>,
) -> Result<Credentials, NanoServiceError> {
    // ── 1. Pull the header ────────────────────────────────────────────────
    let encoded = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| NanoServiceError::new(
            "No credentials provided".into(),
            NanoServiceErrorStatus::Unauthorized,
        ))?
        .to_str()
        .map_err(|_| NanoServiceError::new(
            "Invalid credentials".into(),
            NanoServiceErrorStatus::Unauthorized,
        ))?;

    // ── 2. Expect the “Basic ” scheme prefix ───────────────────────────────
    if !encoded.starts_with("Basic ") {
        return Err(NanoServiceError::new(
            "Invalid credentials".into(),
            NanoServiceErrorStatus::Unauthorized,
        ));
    }

    // ── 3. Decode the `email:password` blob ────────────────────────────────
    let base64_credentials = &encoded[6..];
    let decoded = general_purpose::STANDARD
        .decode(base64_credentials)
        .map_err(|e| NanoServiceError::new(
            e.to_string(),
            NanoServiceErrorStatus::Unauthorized,
        ))?;

    let credentials = String::from_utf8(decoded).map_err(|e| NanoServiceError::new(
        e.to_string(),
        NanoServiceErrorStatus::Unauthorized,
    ))?;

    // ── 4. Split into components ───────────────────────────────────────────
    let mut split = credentials.splitn(2, ':');
    match (split.next(), split.next()) {
        (Some(email), Some(password)) => Ok(Credentials {
            email: email.to_owned(),
            password: password.to_owned(),
        }),
        _ => Err(NanoServiceError::new(
            "Invalid credentials".into(),
            NanoServiceErrorStatus::Unauthorized,
        )),
    }
}
