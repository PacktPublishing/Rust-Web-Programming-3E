use dal::to_do_items::transactions::{
    create::SaveOne,
    get::GetAll
};
use core::api::basic_actions::{
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


pub async fn create<T, X>(token: HeaderToken, body: Json<NewToDoItem>) 
    -> Result<HttpResponse, NanoServiceError>
where
    T: SaveOne + GetAll,
    X: GetUserSession
{
    let session = X::get_user_session(token.unique_id).await?;
    let _ = create_core::<T>(body.into_inner(), session.user_id).await?;
    Ok(HttpResponse::Created().json(get_all_core::<T>(session.user_id).await?))
}
