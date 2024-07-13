use dal::to_do_items::transactions::{
    delete::DeleteOne,
    get::GetAll
};
use core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use actix_web::{
    HttpResponse,
    HttpRequest
};
use glue::{
    token::HeaderToken,
    errors::{
        NanoServiceError,
        NanoServiceErrorStatus
    }
};
use auth_kernel::user_session::UserSession;


pub async fn delete_by_name<T: DeleteOne + GetAll>(token: HeaderToken, req: HttpRequest) -> Result<HttpResponse, NanoServiceError> {
    let session = UserSession::new(token.unique_id).await?;
    match req.match_info().get("name") {
        Some(name) => {
            delete_core::<T>(name, session.user_id).await?;
        },
        None => {
            return Err(
                NanoServiceError::new(
                    "Name not provided".to_string(),
                    NanoServiceErrorStatus::BadRequest
                )
            )
        }
    };
    Ok(HttpResponse::Ok().json(get_all_core::<T>(session.user_id).await?))
}
