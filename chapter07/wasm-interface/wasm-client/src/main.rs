use wasmtime::{Result, Engine, Linker, Module, Store, Config};
use wasmtime_wasi::preview1::{self, WasiP1Ctx};
use wasmtime_wasi::WasiCtxBuilder;
use std::mem::size_of;
use std::slice::from_raw_parts;
use kernel::{
    SomeDataStruct, 
    ResultPointer
};


// An example of executing a WASIp1 "command"
#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.async_support(true);
    let engine = Engine::new(&config).unwrap();
    let module = Module::from_file(&engine, "../wasm-lib/wasm_lib.wasm").unwrap();

    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    preview1::add_to_linker_async(&mut linker, |t| t).unwrap();
    let pre = linker.instantiate_pre(&module)?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .build_p1();

    let mut store = Store::new(&engine, wasi_ctx);
    let instance = pre.instantiate_async(&mut store).await.unwrap();

    // put the stuff below as a loop in the actor 

    let data_struct = SomeDataStruct {
        names: vec!["name1".to_string(), "name2".to_string()],
        name: "name3".to_string()
    };
    let serialized = bincode::serialize(&data_struct).unwrap();

    // allocate the memory for the input data
    let malloc = instance.get_typed_func::<(i32, i32), i32>(&mut store, "ns_malloc").unwrap();
    let input_data_ptr = malloc.call_async(&mut store, (serialized.len() as i32, 0)).await.unwrap();

    // write the contract to the memory
    let memory = instance.get_memory(&mut store, "memory").unwrap();
    memory.write(&mut store, input_data_ptr as usize, &serialized).unwrap();

    // load and call the entry point
    let entry_point = instance.get_typed_func::<(i32, i32), i32>(&mut store, "entry_point").unwrap();
    let ret = entry_point.call_async(&mut store, (input_data_ptr, serialized.len() as i32)).await.unwrap();

    let mut result_buffer = Vec::with_capacity(size_of::<ResultPointer>());
    for _ in 0..size_of::<ResultPointer>() {
        result_buffer.push(0);
    }
    memory.read(&mut store, ret as usize, &mut result_buffer).unwrap();
    let result_struct = unsafe {
        &from_raw_parts::<ResultPointer>(result_buffer.as_ptr() as *const ResultPointer, 1)[0]
    };

    let mut output_buffer: Vec<u8> = Vec::with_capacity(result_struct.len as usize);
    output_buffer.resize(result_struct.len as usize, 0);

    memory.read(&mut store, result_struct.ptr as usize, &mut output_buffer).unwrap();
    let output = bincode::deserialize::<SomeDataStruct>(&output_buffer).unwrap();
    println!("Output contract: {:?}", output);

    let free = instance.get_typed_func::<(i32, i32, i32), ()>(&mut store, "ns_free").unwrap();
    free.call_async(&mut store, (input_data_ptr, serialized.len() as i32, 0)).await.unwrap();
    free.call_async(&mut store, (result_struct.ptr, result_struct.len, 0)).await.unwrap();
    free.call_async(&mut store, (ret, size_of::<ResultPointer>() as i32, 0)).await.unwrap();
    Ok(())
}
