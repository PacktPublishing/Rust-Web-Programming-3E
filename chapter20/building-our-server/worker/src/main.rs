// worker/src/main.rs
use wasmtime::{Result, Engine, Linker, Module, Store, Config};
use wasmtime_wasi::preview1::{self, WasiP1Ctx};
use wasmtime_wasi::WasiCtxBuilder;
use redis::{AsyncCommands, Client};
use tokio::task;
use kernel::Payload;
use std::io;


#[tokio::main]
async fn main() {
    // Initialize the Redis client
    let redis_client = Client::open(
        "redis://127.0.0.1/"
    ).expect("Failed to connect to Redis");

    // Start the worker
    let redis_task = task::spawn(async move {
        worker(redis_client).await;
    });

    // Wait for the worker to finish (it runs indefinitely)
    let _ = redis_task.await;
}

async fn worker(client: Client) {
    let mut connection = client
        .get_async_connection()
        .await
        .expect("Failed to connect to Redis");

    println!("Worker started. Listening for messages on 'payload_queue'...");

    loop {
        // Use BLPOP to fetch one item from the queue
        match connection.blpop::<&str, Option<(String, String)>>("payload_queue", 0.0).await {
            Ok(Some((_queue_name, payload))) => {
                // Deserialize the JSON payload
                match serde_json::from_str::<Payload>(&payload) {
                    Ok(parsed_payload) => {
                        let (updated_payload, outcome) = match run_worker(parsed_payload).await {
                            Ok(result) => result,
                            Err(err) => {
                                eprintln!("Failed to run worker: {}", err);
                                continue;
                            }
                        };
                        if outcome == 1 {
                            println!("task completed");
                        } else {
                            let payload_json: String = serde_json::to_string(&updated_payload).unwrap();
                            let _: () = connection
                                .rpush::<_, _, ()>("payload_queue", payload_json)
                                .await.unwrap();
                            println!("task not completed");
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to deserialize JSON: {}", err);
                    }
                }
            }
            Ok(None) => {
                // BLPOP with no result (unlikely with blocking timeout 0)
                println!("No payload found. Waiting...");
            }
            Err(err) => {
                eprintln!("Error reading from Redis: {}", err);
            }
        }
    }
}





async fn run_worker(payload: Payload) -> Result<(Payload, i32), wasmtime::Error> {
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config)?;

    // // Create a Wasmtime Engine and Store
    let module = Module::new(&engine, &payload.contract)?;
    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    preview1::add_to_linker_async(&mut linker, |t| t)?;

    let pre = linker.instantiate_pre(&module)?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .build_p1();

    let mut store = Store::new(&engine, wasi_ctx);
    let instance = pre.instantiate_async(&mut store).await?;

    let malloc = instance.get_typed_func::<(i32, i32), i32>(&mut store, "ns_malloc")?;
    let input_data_ptr = malloc.call_async(&mut store, (payload.memory.len() as i32, 0)).await?;

    // write the contract to the memory
    let memory = match instance.get_memory(&mut store, "memory") {
        Some(memory) => memory,
        None => return Err(wasmtime::Error::new(io::Error::new(io::ErrorKind::Other, "failed to get memory"))),
    };
    memory.write(&mut store, input_data_ptr as usize, &payload.memory)?;

    let entry_point = instance.get_typed_func::<(i32, i32), i32>(&mut store, "run_compute")?;
    let ret = entry_point.call_async(&mut store, (input_data_ptr, payload.memory.len() as i32)).await?;

    // Read the updated memory back from WebAssembly
    let mut updated_memory = vec![0u8; payload.memory.len()];
    memory
        .read(&mut store, input_data_ptr as usize, &mut updated_memory)?;

    let return_package = Payload {
        memory: updated_memory,
        contract: payload.contract,
    };
    Ok((return_package, ret))
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[tokio::test]
    async fn test_run_worker() {
        let mut file = File::open("./compute_unit.wasm").unwrap();
        let mut wasm = Vec::new();
        file.read_to_end(&mut wasm).unwrap();
        let mut mem = [0; 8];
        let high_num: i32 = 4;
        let new_bytes = high_num.to_le_bytes();
        for i in 0..4 {
            mem[i] = new_bytes[i];
        }
        let payload = Payload {
            memory: mem.to_vec(),
            contract: wasm,
        };
        let payload = run_worker(payload).await.unwrap();
        println!("outcome: {:?}", payload.1);
        let payload = run_worker(payload.0).await.unwrap();
        println!("outcome: {:?}", payload.1);
        let payload = run_worker(payload.0).await.unwrap();
        println!("outcome: {:?}", payload.1);
        let payload = run_worker(payload.0).await.unwrap();
        println!("outcome: {:?}", payload.1);
    }

}


// linker.func_wrap_async(
//     "env",
//     "redis_get",
//     |mut caller: Caller<'_, _>, (key_ptr, key_len): (i32, i32)| {
//         Box::new(async move {
//             let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
//             let data = memory.data(&caller);

//             // Extract the key from memory
//             let key = &data[key_ptr as usize..(key_ptr + key_len) as usize];
//             let key_str = std::str::from_utf8(key).unwrap();

//             // Connect to Redis and get the value
//             let client = redis::Client::open("redis://127.0.0.1/").unwrap();
//             let mut con = client.get_async_connection().await.unwrap();
//             let value: i32 = con.get(key_str).await.unwrap_or(0);

//             Ok(value)
//         })
//     },
// )?;

// let mut mem = [0; 8];
// let high_num: i32 = 8;
// let new_bytes = high_num.to_le_bytes();
// for i in 0..4 {
//     mem[i] = new_bytes[i];
// }