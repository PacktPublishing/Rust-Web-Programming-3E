use actix_web::{
    web, App, HttpServer, Responder, HttpResponse, HttpRequest
};
use rust_embed::RustEmbed;
use std::path::Path;
use to_do_server::api::views_factory as to_do_views_factory;
use actix_cors::Cors;


async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html")
                      .body(include_str!("../index.html"))
}


#[derive(RustEmbed)]
#[folder = "../frontend/public"]
struct FrontendAssets;


fn serve_frontend_asset(path: String) -> HttpResponse {
    let file = match Path::new(&path).file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return HttpResponse::BadRequest()
                                    .body("404 Not Found")
    };
    match FrontendAssets::get(file) {
        Some(content) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(&file)
            .first_or_octet_stream().as_ref())
            .append_header(
                ("Cache-Control", "public, max-age=604800")
            )
            .body(content.data),
        None => HttpResponse::NotFound().body("404 Not Found")
    }
}


async fn catch_all(req: HttpRequest) -> impl Responder {
    if req.path().contains("/api/") {
        return HttpResponse::NotFound().finish()
    }
    if req.path().contains("frontend/public") {
        return serve_frontend_asset(req.path().to_string())
    }
    let file_type = match mime_guess::from_path(&req.path())
                                 .first_raw() {
        Some(file_type) => file_type,
        None => "text/html"
    };
    if !file_type.contains("text/html") {
        return serve_frontend_asset(req.path().to_string())
    }    
    index().await
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin()
                                  .allow_any_method()
                                  .allow_any_header();
        App::new()
            .configure(to_do_views_factory)
            .wrap(cors)
            .default_service(web::route()
            .to(catch_all))
    })
        .bind("0.0.0.0:8001")?
        .run()
        .await
}

