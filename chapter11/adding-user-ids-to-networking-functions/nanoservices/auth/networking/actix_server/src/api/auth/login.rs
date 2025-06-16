use actix_web::HttpResponse;
use auth_dal::users::transactions::get::GetByEmail;
use glue::errors::NanoServiceError;
use crate::extract_auth::extract_credentials;
use auth_core::api::auth::login::login as core_login;


pub async fn login<T: GetByEmail>(req: actix_web::HttpRequest) 
    -> Result<HttpResponse, NanoServiceError> {
    let credentials = extract_credentials(req).await?;
    let token = core_login::<T>(
        credentials.email, 
        credentials.password
    ).await?;
    Ok(HttpResponse::Ok().json(token))
}  
