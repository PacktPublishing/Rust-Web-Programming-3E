#!/usr/bin/env bash
# builds/run_build.sh

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

terraform init
terraform apply -auto-approve
terraform output -json > ./output.json

IP_ADDRESS=$(jq --raw-output '.ec2_global_ips.value[0][0]' \
output.json)
echo $IP_ADDRESS

echo "waiting for server to be built"
sleep 5
echo "attempting to enter server"

scp -i "~/.ssh/keys/remotebuild.pem" \
-o StrictHostKeyChecking=no \
-r ../nanoservices \
ec2-user@$IP_ADDRESS:/home/ec2-user/nanoservices

rm -rf ../ingress/frontend/node_modules

scp -i "~/.ssh/keys/remotebuild.pem" \
-o StrictHostKeyChecking=no \
-r ../ingress \
ec2-user@$IP_ADDRESS:/home/ec2-user/ingress

scp -i "~/.ssh/keys/remotebuild.pem" \
-o StrictHostKeyChecking=no \
-r ../glue \
ec2-user@$IP_ADDRESS:/home/ec2-user/glue

scp -i "~/.ssh/keys/remotebuild.pem" \
-o StrictHostKeyChecking=no \
../docker-compose.yml \
ec2-user@$IP_ADDRESS:/home/ec2-user/

scp -i "~/.ssh/keys/remotebuild.pem" \
-o StrictHostKeyChecking=no \
../.env \
ec2-user@$IP_ADDRESS:/home/ec2-user/

ssh -i "~/.ssh/keys/remotebuild.pem" \
-o StrictHostKeyChecking=no -t \
ec2-user@$IP_ADDRESS << EOF
    until [ -f ./output.txt ]
    do
        echo "waiting for file"
        sleep 3
    done
    echo "File found"
    sudo curl https://sh.rustup.rs -sSf | bash -s -- -y
    source ~/.cargo/env
    export $(cat .env | xargs)
    cd ingress
    cd frontend
    npm install
    npm run build
    cd ..
    cargo build --release
EOF
