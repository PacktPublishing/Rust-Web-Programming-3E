use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;


async fn await_messages(addr: &str) -> Result<(), String> {
    let socket = TcpStream::connect(addr).await.map_err(
        |e| format!("Failed to connect: {}", e)
    )?;
    let (mut reader, _) = socket.into_split();
    loop {
        let mut num_buf = [0; 4];
        reader.read_exact(&mut num_buf).await.map_err(
            |e| format!("Failed to read length: {}", e)
        )?;
        let num: usize = u32::from_be_bytes(num_buf) as usize;
        println!("{:?}", num);
        if num == 0 {
            break;
        }
    }
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), String> {
    await_messages("127.0.0.1:9000").await?;
    Ok(())
}
