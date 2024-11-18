use to_do_core::api::basic_actions::get::{
    get_all as get_all_core,
    get_by_name as get_by_name_core
};
use actix_web::{
    HttpResponse,
    HttpRequest
};
use glue::errors::{
    NanoServiceError,
    NanoServiceErrorStatus
};



/// Gets all tasks.
/// 
/// # Returns
/// An `HttpResponse` with a JSON body containing all tasks.
pub async fn get_all() -> Result<HttpResponse, NanoServiceError> {
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}

/// Gets a task by name.
/// 
/// # Arguments
/// * `req`: The request the extract the name from the URL
/// 
/// # Resurns
/// An `HttpResponse` with a JSON body containing of the task specified in the URL
pub async fn get_by_name(req: HttpRequest) -> Result<HttpResponse, NanoServiceError> {
    let name = match req.match_info().get("name") {
        Some(name) => name,
        None => {
            return Err(
                NanoServiceError::new(
                    "Name not provided".to_string(),
                    NanoServiceErrorStatus::BadRequest
                )
            )
        }
    };
    Ok(HttpResponse::Ok().json(get_by_name_core(name).await?))
}

