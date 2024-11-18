#[macro_use] extern crate rocket;
use rocket::http::{ContentType, Status};
use rocket::response::content::RawHtml;
use rocket::response::Responder;
use rust_embed::RustEmbed;
use std::path::PathBuf;
use std::io::Cursor;
use rocket::response::Response;
use rocket::response;
use rocket::Request;
use std::marker::PhantomData;
use std::net::Ipv4Addr;

use rocket::http::Header;
use rocket::fairing::{Fairing, Info, Kind};

#[derive(RustEmbed)]
#[folder = "../../frontend/public"]
struct FrontendAssets;


struct CustomResponder<'r> {
    status: Status,
    content_type: ContentType,
    data: Cursor<Vec<u8>>,
    data_len: usize,
    _phantom: PhantomData<&'r ()>,
}

impl<'r> Responder<'r, 'static> for CustomResponder<'r> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(self.status)
            .header(self.content_type)
            .sized_body(self.data_len, self.data)
            .ok()
    }
}

#[get("/")]
async fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../index.html"))
}

#[get("/<path..>")]
async fn serve_frontend_asset<'r>(path: PathBuf) -> Result<CustomResponder<'r>, Status> {
    let file = match path.file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return Err(Status::NotFound),
    };

    if let Some(content) = FrontendAssets::get(file) {
        let content_type: ContentType = ContentType::from_extension(file.split('.').last().unwrap_or("")).unwrap_or(ContentType::Any);
        let raw_data = content.data.to_vec();
        let data_len = raw_data.len();
        let data = Cursor::new(raw_data);
        Ok(CustomResponder {
            status: Status::Ok,
            content_type,
            data,
            data_len,
            _phantom: PhantomData,
        })
    } else {
        Err(Status::NotFound)
    }
}

#[catch(404)]
async fn catch_all<'r>(req: &rocket::Request<'_>) -> Result<CustomResponder<'r>, Status> {
    let path = req.uri().path().to_string();
    
    if path.contains("/api/") {
        return Err(Status::NotFound);
    }

    if path.contains("frontend/public") {
        return serve_frontend_asset(PathBuf::from(path)).await;
    }
    
    let file_type = mime_guess::from_path(path.clone()).first_raw().unwrap_or("text/html");
    if !file_type.contains("text/html") {
        return serve_frontend_asset(PathBuf::from(path)).await;
    }
    let raw_data = include_str!("../index.html").as_bytes().to_vec();
    let data_len = raw_data.len();
    let data = Cursor::new(raw_data);
    Ok(CustomResponder {
        status: Status::Ok,
        content_type: ContentType::HTML,
        data,
        data_len,
        _phantom: PhantomData,
    })
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }
    async fn on_response<'r>(
        &self, _request: &'r Request<'_>, response: &mut Response<'r>
    ) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin", "*"
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"
        ));
        response.set_header(
            Header::new("Access-Control-Allow-Headers", "*"
        ));
        response.set_header(
            Header::new("Access-Control-Allow-Credentials", "true"
        ));
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let config = rocket::Config {
        port: 8001,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let rocket = rocket::custom(&config);
    let rocket = rocket.mount("/", routes![index, serve_frontend_asset]);
    let rocket = rocket.register("/", catchers![catch_all]);
    let rocket = rocket.attach(CORS);
    rocket.launch().await?;
    Ok(())
}
