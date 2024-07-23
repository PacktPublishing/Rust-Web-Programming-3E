use auth_dal::users::transactions::get::GetByEmail;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::token::HeaderToken;


pub async fn login<T: GetByEmail>(email: String, password: String) -> Result<String, NanoServiceError> {
    let user = T::get_by_email(email).await?;
    let outcome = user.verify_password(password)?;
    if outcome {
        Ok(HeaderToken{
            unique_id: user.unique_id
        }.encode()?)
    } else {
        Err(NanoServiceError::new(
            "Invalid password".to_string(),
            NanoServiceErrorStatus::Unauthorized
        ))
    }
}
