#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..
export $(cat .env | xargs)
cd frontend
npm install
npm run build
cd ..
cargo clean
cargo run
