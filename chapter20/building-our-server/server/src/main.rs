// File: server/src/main.rs
use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use redis::{AsyncCommands, Client};
use std::net::SocketAddr;
use std::sync::Arc;
use kernel::Payload;


#[derive(Clone)]
struct AppState {
    redis_client: Client,
}

#[tokio::main]
async fn main() {
    // Initialize the Redis client
    let redis_client = Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");

    // Shared app state wrapped in Arc
    let app_state = Arc::new(AppState { redis_client });

    // Define the Axum router
    let app: Router<_> = Router::new()
    .route("/submit", post(write_to_redis))
    .with_state(app_state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}


async fn write_to_redis(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Payload>,
) -> Result<&'static str, (axum::http::StatusCode, String)> {
    let mut connection = state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|err| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )
        })?;

    // Serialize the payload as JSON to store it in Redis
    let payload_json: String = serde_json::to_string(&payload)
        .map_err(|err| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )
        })?;

    // Push the JSON string to the Redis queue
    let _: () = connection
        .lpush::<_, _, ()>("payload_queue", payload_json)
        .await
        .map_err(|err| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )
        })?;

    Ok("Payload submitted to Redis queue")
}
