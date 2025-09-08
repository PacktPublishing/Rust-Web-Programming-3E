use hyper::{
    body::{Bytes, Incoming},
    header,
    Request, Response,
};
use http_body_util::Full;
use glue::{
    errors::{NanoServiceError, NanoServiceErrorStatus},
    safe_eject,
};

/// GET /api/v1/auth/logout
///
/// Sends a small HTML page that removes the `token` from
/// `localStorage` and redirects the browser back to the site root.
pub async fn logout(
    _req: Request<Incoming>,             // request isn’t used, but Hyper still passes it in
) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    // ── 1. Static HTML payload ─────────────────────────────────────────────
    const HTML_LOGOUT: &str = r#"
        <html>
            <script>
                /* clear the client-side token */
                localStorage.removeItem('token');
                /* redirect to the origin (/) */
                window.location.replace(document.location.origin);
            </script>
        </html>
    "#;

    // ── 2. Build and return the response (mirrors the style in `login`) ────
    safe_eject!(
        Response::builder()
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .status(200)
            .body(Full::new(Bytes::from_static(HTML_LOGOUT.as_bytes()))),
        NanoServiceErrorStatus::Unknown
    )
}
