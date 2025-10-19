use dal::to_do_items::transactions::{
    update::UpdateOne,
    get::GetAll
};
use todo_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use dal::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use glue::token::HeaderToken;
use actix_web::{
    HttpResponse,
    web::Json
};
use auth_kernel::user_session::transactions::get::GetUserSession;


pub async fn update<T, X>(token: HeaderToken, body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> 
where
    T: UpdateOne + GetAll,
    X: GetUserSession
{
    let session = X::get_user_session(token.unique_id).await?;
    let _ = update_core::<T>(body.into_inner(), session.user_id).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>(session.user_id).await?))
}
