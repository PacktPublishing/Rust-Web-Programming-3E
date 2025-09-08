//
// #[get("/")]
// fn index() -> &'static str { "Hello Rocket" }
//
// #[launch]
// fn rocket() -> _ {
//     tracing_subscriber::fmt().init();
//
//     rocket::build()
//         .attach(RocketLogger)
//         .mount("/", routes![index])
// }
// -----------------------------------------------------------------------------
// Cargo.toml snippet:
// -----------------------------------------------------------------------------
// [dependencies]
// rocket = { version = "0.5", features = ["tls"] }
// tracing = "0.1"
// tracing-subscriber = "0.3"
// -----------------------------------------------------------------------------
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use tracing::info;

/// A simple request/response logger for Rocket.
#[derive(Clone, Copy, Debug, Default)]
pub struct RocketLogger;

#[rocket::async_trait]
impl Fairing for RocketLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request / response logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    // Nothing to do on the request _before_ it’s handled, but we could stamp a
    // request ID or start a timer here if desired.
    async fn on_request(&self, _req: &mut Request<'_>, _data: &mut Data<'_>) {}

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        info!(
            "Request: {} {} {}",
            req.method(),
            req.uri(),
            res.status()
        );
    }
}