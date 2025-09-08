use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{header, Response, Request, body::Incoming};
use auth_core::api::users::create::{
    create as create_core,
    CreateUser
};
use auth_dal::users::schema::NewUser;
use glue::hyper_utils::extract_body::extract_body;
use auth_dal::users::transactions::create::SaveOne;


pub async fn create<T: SaveOne>(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let new_user = extract_body::<CreateUser>(req).await?;
    create_core::<T>(new_user).await?;
    safe_eject!(
        Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .status(201)
                .body(Full::new(Bytes::from_static(b"{}"))),
        NanoServiceErrorStatus::Unknown
    )
}
