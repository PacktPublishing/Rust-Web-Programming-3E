use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use kernel::KvMessage;


async fn process(mut socket: TcpStream, state: Arc<Mutex<HashMap<Vec<u8>, Vec<u8>>>>) {
    // Read the length of the incoming message
    let mut len_buf = [0; 4];
    if socket.read_exact(&mut len_buf).await.is_err() {
        eprintln!("Failed to read length");
        return;
    }
    
    let len = u32::from_be_bytes(len_buf) as usize;
    println!("Expecting {} bytes", len);

    // Read the actual message data
    let mut buffer = vec![0; len];
    if socket.read_exact(&mut buffer).await.is_err() {
        eprintln!("Failed to read message");
        return;
    }

    // Deserialize the message using bincode
    let message: KvMessage = bincode::deserialize(&buffer).unwrap();
    let return_message = match message {
        KvMessage::Get(key) => {
            let value = state.lock().unwrap().get(&key).cloned();
            KvMessage::ReturnValue(value)
        },
        KvMessage::Put((key, value)) => {
            state.lock().unwrap().insert(key, value);
            KvMessage::Success(true)
        },
        KvMessage::Del(key) => {
            state.lock().unwrap().remove(&key);
            KvMessage::Success(true)
        },
        _ => {
            KvMessage::Error("Invalid message type".to_string())
        }
    };
    let return_message_data = return_message.package();
    if let Err(e) = socket.write_all(&return_message_data).await {
        eprintln!("Failed to write to socket; err = {:?}", e);
    }
}



#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    let state = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let state_ref = state.clone();
        tokio::spawn(async move {
            println!("new connection incoming");
            process(socket, state_ref).await;
        });
    }
}
