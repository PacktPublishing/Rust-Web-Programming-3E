use axum::response::Html;


pub async fn logout() -> Html<&'static str> {
    Html(r#"
        <html>
            <script>
                // wipe the token stored by your SPA
                localStorage.removeItem('token');
                // bounce the user back to the site root
                window.location.replace(document.location.origin);
            </script>
        </html>
    "#)
}
