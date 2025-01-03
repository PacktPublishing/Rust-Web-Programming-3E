# File: wasm-lib/scripts/build.sh
#!/bin/bash
# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

cargo build --release --target wasm32-wasi

cp target/wasm32-wasi/release/wasm_lib.wasm ./wasm_lib.wasm
wasm2wat ./wasm_lib.wasm > ./wasm_lib.wat
