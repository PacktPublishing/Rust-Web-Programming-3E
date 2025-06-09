#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..


# Default framework is actix if no argument is passed
framework=${1:-actix}

# Build and run the specified framework server
case $framework in
  actix)
    echo "Building and running Actix server..."
    cargo build -p to-do-actix-server
    cargo run -p to-do-actix-server &
    PID=$!
    ;;
  axum)
    echo "Building and running Axum server..."
    cargo build -p to-do-axum-server
    cargo run -p to-do-axum-server &
    PID=$!
    ;;
  hyper)
    echo "Building and running Hyper server..."
    cargo build -p to-do-hyper-server
    cargo run -p to-do-hyper-server &
    PID=$!
    ;;
  rocket)
    echo "Building and running Rocket server..."
    cargo build -p to-do-rocket-server
    cargo run -p to-do-rocket-server &
    PID=$!
    ;;
  *)
    echo "Unknown framework: $framework"
    echo "Usage: $0 [actix|axum|hyper|rocket]"
    exit 1
    ;;
esac
sleep 1


rm tasks.json
rm output.txt
cat <<EOF > tasks.json
{}
EOF

echo "Server started with PID: $PID"

curl -X POST http://127.0.0.1:8080/api/v1/create \
     -H "Content-Type: application/json" \
     -d '{"title": "writing", "status": "PENDING"}' >> output.txt

echo "" >> output.txt

curl -X POST http://127.0.0.1:8080/api/v1/create \
     -H "Content-Type: application/json" \
     -d '{"title": "coding", "status": "PENDING"}' >> output.txt

echo "" >> output.txt

curl -X DELETE http://127.0.0.1:8080/api/v1/delete/coding >> output.txt

echo "" >> output.txt

curl -X PATCH http://127.0.0.1:8080/api/v1/update \
     -H "Content-Type: application/json" \
     -H "token: some token" \
     -d '{"title": "writing", "status": "DONE"}' >> output.txt

kill $PID
