use axum::http::{header, Request};
use base64::{engine::general_purpose, Engine};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

/// Pulls Basic-Auth credentials out of an Axum/Hyper request.
pub fn extract_credentials<B>(
    req: &Request<B>,
) -> Result<Credentials, NanoServiceError> {
    // ── 1. Grab the header ────────────────────────────────────────────────
    let header_value = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| NanoServiceError::new(
            "No credentials provided".to_string(),
            NanoServiceErrorStatus::Unauthorized,
        ))?;

    let encoded = header_value.to_str().map_err(|_| NanoServiceError::new(
        "Invalid credentials".to_string(),
        NanoServiceErrorStatus::Unauthorized,
    ))?;

    // ── 2. Expect the “Basic ” prefix ──────────────────────────────────────
    if !encoded.starts_with("Basic ") {
        return Err(NanoServiceError::new(
            "Invalid credentials".to_string(),
            NanoServiceErrorStatus::Unauthorized,
        ));
    }

    // ── 3. Base-64 decode the `email:password` blob ────────────────────────
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

    // ── 4. Split into email / password ─────────────────────────────────────
    let mut split = credentials.splitn(2, ':');
    match (split.next(), split.next()) {
        (Some(email), Some(password)) => Ok(Credentials {
            email: email.to_owned(),
            password: password.to_owned(),
        }),
        _ => Err(NanoServiceError::new(
            "Invalid credentials".to_string(),
            NanoServiceErrorStatus::Unauthorized,
        )),
    }
}
