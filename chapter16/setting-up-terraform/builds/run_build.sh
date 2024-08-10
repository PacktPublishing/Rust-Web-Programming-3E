#!/usr/bin/env bash
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

rm -rf ../web_app/target/
scp -i "~/.ssh/keys/remotebuild.pem" -r
../web_app ec2-user@$1:/home/ec2-user/web_app