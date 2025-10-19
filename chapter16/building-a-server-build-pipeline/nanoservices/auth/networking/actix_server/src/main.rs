use actix_web::{App, HttpServer};

mod api;
mod extract_auth;
use auth_dal::migrations::run_migrations;
use glue::logger::{
    logger::init_logger,
    network_wrappers::actix_web::ActixLogger
};
use actix_cors::Cors;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    init_logger();
    run_migrations().await;
    
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin()
                                  .allow_any_method()
                                  .allow_any_header();
        App::new().wrap(ActixLogger)
                  .wrap(cors)
                  .configure(api::views_factory)
    })
    .workers(4)
    .bind("127.0.0.1:8081")?
    .run()
    .await
}