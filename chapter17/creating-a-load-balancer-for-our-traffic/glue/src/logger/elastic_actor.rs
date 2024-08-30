use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use serde_json::json;
use reqwest::{Client, Body};
use chrono::Utc;
use once_cell::sync::Lazy;
use serde::Serialize;


#[derive(Debug, Serialize)]
struct LogMessage {
    level: String,
    message: String,
}


pub async fn send_log(level: &str, message: &str) {
    static LOG_CHANNEL: Lazy<Sender<LogMessage>> = Lazy::new(|| {
        let (tx, rx) = mpsc::channel(100);
        tokio::spawn(async move {
            elastic_actor(rx).await;
        });
        tx
    });
    LOG_CHANNEL.send(LogMessage {
        level: level.to_string(),
        message: message.to_string(),
    }).await.unwrap();
}


async fn elastic_actor(mut rx: Receiver<LogMessage>) {

    let elastic_url = std::env::var("ELASTICSEARCH_URL").unwrap();

    let client = Client::new();
    while let Some(log) = rx.recv().await {
        let body = json!({
            "level": log.level,
            "message": log.message,
            "timestamp": Utc::now().to_rfc3339()
        });
        let body = Body::from(serde_json::to_string(&body).unwrap());

        match client.post(&elastic_url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(body)
            .send()
            .await
        {
            Ok(result) => {},
            Err(e) => {
                eprintln!("Failed to send log to Elasticsearch: {}", e);
            }
        }
    }
}