#!/usr/bin/env bash

curl -X POST http://0.0.0.0:8001/api/v1/users/create \
-H "Content-Type: application/json" \
-d '{
  "email": "test@gmail.com",
  "password": "password"
}'

curl -u test@gmail.com:password -X GET http://0.0.0.0:8001/api/v1/auth/login
