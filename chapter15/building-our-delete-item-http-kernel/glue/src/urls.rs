use super::errors::{NanoServiceError, NanoServiceErrorStatus};


pub fn get_to_do_url() -> Result<String, NanoServiceError> {
    std::env::var("TO_DO_API_URL").map_err(|e|{
        NanoServiceError::new(e.to_string(), 
        NanoServiceErrorStatus::BadRequest)
    })
}
