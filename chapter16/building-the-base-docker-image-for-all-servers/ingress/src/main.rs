//! This server is responsible for managing the tagging of objects and the creation of records
//! for objects in the system.
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use rust_embed::RustEmbed;
use std::path::Path;
use to_do_server::api::views_factory as to_do_views_factory;
use auth_server::api::views_factory as auth_views_factory;
use actix_cors::Cors;
use to_do_dal::migrations::run_migrations as run_todo_migrations;
use auth_dal::migrations::run_migrations as run_auth_migrations;
use glue::logger::{
    logger::init_logger,
    network_wrappers::actix_web::ActixLogger
};


/// Serves the HTML file for the frontend which will load the bundle.js file. 
async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(include_str!("../index.html"))
}


/// Catches all requests that are not handled by the other routes. If the route does not have "/api/" in it, then
/// it will check to see if the request is for static files from the frontend or admin frontend. If it is, then it will
/// serve the file. Otherwise, it will serve the index.html file for the frontend or the index_admin.html file for the
/// admin frontend.
/// 
/// # Arguments
/// * `req` - The request that is being handled.
/// 
/// # Returns
/// bytes of a file
async fn catch_all(req: HttpRequest) -> impl Responder {
    if req.path().contains("/api/") {
        return HttpResponse::NotFound().finish()
    }
    if req.path().contains("frontend/public") {
        return serve_frontend_asset(req.path().to_string())
    }
    let file_type = match mime_guess::from_path(&req.path()).first_raw() {
        Some(file_type) => file_type,
        None => "text/html"
    };
    if !file_type.contains("text/html") {
        return serve_frontend_asset(req.path().to_string())
    }
    index().await
}


/// Embeds the frontend files into the binary.
#[derive(RustEmbed)]
#[folder = "./frontend/public"]
struct FrontendAssets;


/// Serves the frontend files from the binary.
/// 
/// # Arguments
/// * `path` - The path from the request.
/// 
/// # Returns
/// a http response with the bytes of the file
fn serve_frontend_asset(path: String) -> HttpResponse {
    let file = match Path::new(&path).file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return HttpResponse::BadRequest().body("404 Not Found")
    };
    match FrontendAssets::get(file) {
        Some(content) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(&file).first_or_octet_stream().as_ref())
            .append_header(("Cache-Control", "public, max-age=604800"))
            .body(content.data),
        None => HttpResponse::NotFound().body("404 Not Found")
    }
}


#[tokio::main]
async fn main() -> std::io::Result<()> {

    init_logger();
    run_todo_migrations().await;
    run_auth_migrations().await;

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        App::new()
            .configure(auth_views_factory)
            .configure(to_do_views_factory)
            .wrap(cors)
            .wrap(ActixLogger)
            .default_service(web::route().to(catch_all))
    })
        .bind("0.0.0.0:8001")?
        .run()
        .await
}
