#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..
cd http-server

# check if on linux or mac
if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Linux detected"
  sudo apt install openssl
#   openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Mac detected"
  brew install openssl
#   openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365
else
  echo "OS not supported"
fi

openssl genrsa -out key.pem 2048
openssl req -new -x509 -key key.pem -out cert.pem -days 365
