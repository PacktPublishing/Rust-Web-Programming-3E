#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

cargo build -p compute-unit --target wasm32-wasi --release

if [ -f ./client/compute_unit.wasm ]; then
    echo "remove computing unit in worker"
    rm ./client/compute_unit.wasm
fi

cp target/wasm32-wasi/release/compute_unit.wasm \
./client/compute_unit.wasm
