use rocket::Request;
use base64::{engine::general_purpose, Engine};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

/// Extract `Authorization: Basic …` credentials from a Rocket request.
///
/// ### Example use inside a route
/// ```rust
/// #[get("/secret")]
/// fn secret(req: &Request<'_>) -> Result<String, NanoServiceError> {
///     let creds = extract_credentials(req)?;
///     Ok(format!("Welcome, {}!", creds.email))
/// }
/// ```
pub fn extract_credentials<'r>(
    req: &'r Request<'_>,
) -> Result<Credentials, NanoServiceError> {
    // ── 1. Get the header ───────────────────────────────────────────────────
    let encoded = req
        .headers()
        .get_one("Authorization")
        .ok_or_else(|| NanoServiceError::new(
            "No credentials provided".into(),
            NanoServiceErrorStatus::Unauthorized,
        ))?;

    // ── 2. Expect the “Basic ” scheme prefix ────────────────────────────────
    if !encoded.starts_with("Basic ") {
        return Err(NanoServiceError::new(
            "Invalid credentials".into(),
            NanoServiceErrorStatus::Unauthorized,
        ));
    }

    // ── 3. Base-64 decode the `email:password` blob ─────────────────────────
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

    // ── 4. Split into email / password ──────────────────────────────────────
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
