#!/usr/bin/env bash
# scripts/run_tests.sh

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

# Remove the log directory if it exists
if [ -d "./logs" ]; then
  echo "Removing existing log directory: ./logs"
  rm -rf "./logs"
fi
mkdir logs

# Function to check if PostgreSQL is ready
function check_postgres_ready() {
  docker compose exec -T postgres pg_isready \
  -U username -d to_do
}

export $(cat .env | xargs)

cargo test -p core > ./logs/core.log
cargo test -p actix_server > ./logs/actix_server.log

docker compose up -d

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
until check_postgres_ready; do
  echo -n "."
  sleep 1
done

echo "PostgreSQL is ready."

export TO_DO_MAX_CONNECTIONS=1
cargo test -p dal --features sqlx-postgres \
-- --test-threads=1 > ./logs/dal.log

# cargo build -p actix_server
# cargo run -p actix_server &
# to_do_server_pid=$!

cargo build -p actix_server
cargo run -p actix_server > ./logs/todo_server.log &
to_do_server_pid=$!

cargo build -p auth_actix_server
cargo run -p auth_actix_server > ./logs/auth_server.log &
auth_server_pid=$!
sleep 1

cargo test -p to-do-atomic-http-tests -- \
--test-threads=1 --nocapture \
> ./logs/to-do-atomic-http-tests.log

cargo test -p to-do-http-workflow-tests -- \
--test-threads=1 --nocapture \
> ./logs/to-do-http-workflow-tests.log

docker compose down
kill $to_do_server_pid

kill $to_do_server_pid
kill $auth_server_pid
