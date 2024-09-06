use tokio::net::TcpListener;
use std::error::Error;
use std::env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // process the TCP request
            println!(
                "Accepted connection from: {}", 
                socket.peer_addr().unwrap()
            );
        });
    }
}
