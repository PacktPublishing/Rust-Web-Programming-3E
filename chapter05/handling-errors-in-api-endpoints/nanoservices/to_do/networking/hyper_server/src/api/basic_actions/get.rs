use hyper::body::Bytes;
use hyper::{header, Response, StatusCode};
use to_do_core::api::basic_actions::get::get_all as get_all_core;
use http_body_util::Full;


/// Gets all tasks.
/// 
/// # Returns
/// A `Response` with a body containing all tasks or an error message.
pub async fn get_all() -> Response<Full<Bytes>> {
    let outcome = match get_all_core().await {
        Ok(items) => {
            let json_body = serde_json::to_string(&items).unwrap();
            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Full::new(Bytes::from(json_body)))
                .unwrap()
        }
        Err(e) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::new(Bytes::from(e.to_string())))
                .unwrap()
        }
    };
    outcome
}