use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};


async fn process(socket: TcpStream) {
    let (_, mut writer) = socket.into_split();
    for i in 1u32..10 {
        let bytes = i.to_be_bytes();
        writer.write_all(&bytes).await.unwrap();
    }
    writer.write_all(&[0, 0, 0, 0]).await.unwrap();
}



#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(
        "127.0.0.1:9000"
    ).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            println!("new connection incoming");
            process(socket).await;
        });
    }
}
