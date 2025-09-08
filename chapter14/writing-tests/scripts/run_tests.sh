#!/usr/bin/env bash
# scripts/run_tests.sh

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

cargo test -p to-do-core
cargo test -p to-do-actix-server
