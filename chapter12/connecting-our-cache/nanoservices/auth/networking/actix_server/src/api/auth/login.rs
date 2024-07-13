use actix_web::HttpResponse;
use auth_dal::users::transactions::get::GetByEmail;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use crate::extract_auth::extract_credentials;
use auth_core::api::auth::login::login as core_login;
use cache_client::login as cache_login;


pub async fn login<T: GetByEmail>(req: actix_web::HttpRequest) -> Result<HttpResponse, NanoServiceError> {
    let credentials = extract_credentials(req).await?;
    let token = core_login::<T>(credentials.email.clone(), credentials.password).await?;

    let user = T::get_by_email(credentials.email).await?;
    let url = std::env::var("CACHE_API_URL").map_err(|e|{
        NanoServiceError::new(
            e.to_string(), 
            NanoServiceErrorStatus::Unknown
        )
    })?;

    let _ = cache_login(
        &url,
        &user.unique_id,
        20,
        user.id
    ).await?;
    Ok(HttpResponse::Ok().json(token))
}