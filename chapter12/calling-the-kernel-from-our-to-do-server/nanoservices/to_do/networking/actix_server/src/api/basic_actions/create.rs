use to_do_core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use glue::errors::NanoServiceError;
use actix_web::{
    HttpResponse,
    web::Json
};
use to_do_dal::to_do_items::schema::NewToDoItem;
use to_do_dal::to_do_items::transactions::{
    create::SaveOne,
    get::GetAll
};
use glue::token::HeaderToken;
use auth_kernel::user_session::transactions::get::GetUserSession;


/// Creates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn create<T, X>(
    token: HeaderToken, 
    body: Json<NewToDoItem>
) -> Result<HttpResponse, NanoServiceError>
where
    T: SaveOne + GetAll,
    X: GetUserSession
{
    let session = X::get_user_session(
        token.unique_id
    ).await?;
    let _ = create_core::<T>(
        body.into_inner(), 
        session.user_id
    ).await?;
    Ok(HttpResponse::Created().json(
        get_all_core::<T>(session.user_id).await?
    ))
}
