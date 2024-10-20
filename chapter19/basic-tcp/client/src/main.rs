use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use kernel::KvMessage;


async fn send_message(message: KvMessage, addr: &str) -> Result<KvMessage, String> {
    let mut socket = TcpStream::connect(addr).await.map_err(
        |e| format!("Failed to connect: {}", e)
    )?;

    let data = message.package();
    socket.write_all(&data).await.map_err(
        |e| format!("Failed to write to socket: {}", e)
    )?;

    let mut len_buf = [0; 4];
    if socket.read_exact(&mut len_buf).await.is_err() {
        return Err("Failed to read length".to_string());
    }
    
    let len = u32::from_be_bytes(len_buf) as usize;

    // Read the actual message data
    let mut buffer = vec![0; len];
    if socket.read_exact(&mut buffer).await.is_err() {
        return Err("Failed to read message".to_string());
    }

    let message: KvMessage = bincode::deserialize(&buffer).map_err(
        |e| format!("Failed to deserialize message: {}", e)
    )?;
    return Ok(message)
}


#[tokio::main]
async fn main() -> Result<(), String> {
    println!("{:?}", send_message(
        KvMessage::Get(b"key".to_vec()), 
        "127.0.0.1:9000"
    ).await?);
    println!("{:?}", send_message(
        KvMessage::Put((b"key".to_vec(), b"value".to_vec())), 
        "127.0.0.1:9000"
    ).await?);
    println!("{:?}", send_message(
        KvMessage::Get(b"key".to_vec()), 
        "127.0.0.1:9000"
    ).await?);
    println!("{:?}", send_message(
        KvMessage::Del(b"key".to_vec()), 
        "127.0.0.1:9000"
    ).await?);
    Ok(())
}
