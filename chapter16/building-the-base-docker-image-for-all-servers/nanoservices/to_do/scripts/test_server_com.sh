#!/usr/bin/env bash
# nanoservices/to_do/scripts/test_server_com.sh

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ../../../

docker-compose build
docker-compose up -d
sleep 1

export $(cat ./nanoservices/to_do/.env | xargs)

cargo build \
--manifest-path ./nanoservices/to_do/networking/actix_server/Cargo.toml \
--features auth-http \
--release \
--no-default-features

cargo build \
--manifest-path ./nanoservices/auth/networking/actix_server/Cargo.toml \
--release

cargo run \
--manifest-path ./nanoservices/to_do/networking/actix_server/Cargo.toml \
--features auth-http \
--release --no-default-features &
TO_DO_PID=$!

cargo run \
--manifest-path ./nanoservices/auth/networking/actix_server/Cargo.toml \
--release &
AUTH_PID=$!

sleep 2
curl -X POST http://127.0.0.1:8081/api/v1/users/create \
-H "Content-Type: application/json" \
-d '{
  "email": "test@gmail.com",
  "password": "password"
}'

token=$(curl \
-u test@gmail.com:password \
-X GET http://127.0.0.1:8081/api/v1/auth/login)
token=$(echo "$token" | tr -d '\r\n' | sed 's/^"//' | sed 's/"$//')

response=&(curl -X POST http://127.0.0.1:8080/api/v1/create \
-H "Content-Type: application/json" \
-H "token: $token" \
-d '{
  "title": "code",
  "status": "PENDING"
}')

sleep 1
echo $response
sleep 2

kill $TO_DO_PID
kill $AUTH_PID
docker-compose down
