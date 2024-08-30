#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

cd rust_interface
sudo wasm-pack build --target web 
cp pkg/rust_interface_bg.wasm
