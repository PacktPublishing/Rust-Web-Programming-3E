use rocket::{
    get,
    http::Status,
    response::content::RawHtml,
};

/// GET /logout
///
/// Sends a tiny HTML page that wipes the `token` from
/// `localStorage` and bounces the browser back to the site root.
#[get("/logout")]
pub fn logout() -> (Status, RawHtml<&'static str>) {
    (
        Status::Ok,
        RawHtml(r#"
            <html>
                <script>
                    /* clear the client-side token */
                    localStorage.removeItem('token');
                    /* redirect to the origin (/) */
                    window.location.replace(document.location.origin);
                </script>
            </html>
        "#),
    )
}
