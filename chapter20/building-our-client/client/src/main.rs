// client/src/main.rs
use reqwest::Client;
use kernel::Payload;
use std::fs::File;
use std::io::Read;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./compute_unit.wasm")?;
    let mut wasm = Vec::new();
    file.read_to_end(&mut wasm)?;
    let server_url = "http://127.0.0.1:3000/submit";
    let client = Client::new();

    let mut memory = [0; 8];
    let high_num: i32 = 4;
    let new_bytes = high_num.to_le_bytes();
    for i in 0..4 {
        memory[i] = new_bytes[i];
    }
    let payload = Payload {
        memory: memory.to_vec(),
        contract: wasm,
    };

    for _ in 0..3 {
        let response = client
            .post(server_url)
            .json(&payload)
            .send()
            .await;
        match response {
            Ok(res) if res.status().is_success() => {
                let text = res.text().await.unwrap_or_else(|_| 
                    "No response body".to_string()
                );
                println!("Server response: {}", text);
            }
            Ok(res) => {
                eprintln!("Request failed with status: {}", res.status());
                if let Ok(text) = res.text().await {
                    eprintln!("Response body: {}", text);
                }
            }
            Err(err) => {
                eprintln!("Failed to send request: {}", err);
            }
        }
    }
    Ok(())
}
