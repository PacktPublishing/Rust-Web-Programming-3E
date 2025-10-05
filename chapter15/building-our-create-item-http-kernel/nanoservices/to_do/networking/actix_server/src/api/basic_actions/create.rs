use dal::to_do_items::transactions::{
    create::SaveOne,
    get::GetAll
};
use todo_core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use dal::to_do_items::schema::NewToDoItem;
use glue::{
    errors::NanoServiceError,
    token::HeaderToken
};
use actix_web::{
    HttpResponse,
    web::Json
};
use auth_kernel::user_session::transactions::get::GetUserSession;
use glue::logger::logger::log_info;


pub async fn create<T, X>(token: HeaderToken, body: Json<NewToDoItem>) 
    -> Result<HttpResponse, NanoServiceError>
where
    T: SaveOne + GetAll,
    X: GetUserSession
{
    log_info("Creating new to do item").await;
    let session = X::get_user_session(token.unique_id).await?;
    log_info("User session found").await;
    let _ = create_core::<T>(body.into_inner(), session.user_id).await?;
    log_info("To do item created").await;
    Ok(HttpResponse::Created().json(get_all_core::<T>(session.user_id).await?))
}
