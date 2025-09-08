#!/usr/bin/env bash
# scripts/run_tests.sh

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

# scripts/run_tests.sh
if [ -d "./logs" ]; then
  echo "Removing existing log directory: ./logs"
  rm -rf "./logs"
fi
mkdir logs

function check_postgres_ready() {
  docker compose exec -T postgres pg_isready \
  -U username -d to_do
}

export $(cat .env | xargs)

cargo test -p to-do-core > ./logs/to-do-core.log
cargo test -p to-do-actix-server > ./logs/to-do-actix-server.log

docker compose up -d

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
until check_postgres_ready; do
  echo -n "."
  sleep 1
done

echo "PostgreSQL is ready."


export TO_DO_MAX_CONNECTIONS=1
cargo test -p to-do-dal --features sqlx-postgres \
-- --test-threads=1 > ./logs/to-do-dal.log

docker compose down
