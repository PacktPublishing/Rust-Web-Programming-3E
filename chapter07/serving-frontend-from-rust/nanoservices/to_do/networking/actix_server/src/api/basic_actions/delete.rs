use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use actix_web::{
    HttpResponse,
    HttpRequest
};
use glue::errors::{
    NanoServiceError,
    NanoServiceErrorStatus
};


/// Deletes an item from the to-do list by name.
/// 
/// # Arguments
/// - `req` - The HTTP request.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn delete_by_name(req: HttpRequest) -> Result<HttpResponse, NanoServiceError> {
    match req.match_info().get("name") {
        Some(name) => {
            delete_core(name).await?;
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
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
