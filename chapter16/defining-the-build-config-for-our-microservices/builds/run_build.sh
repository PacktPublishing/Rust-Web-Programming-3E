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

scp -i "~/.ssh/keys/remotebuild.pem" \
-o StrictHostKeyChecking=no \
../Cargo.toml \
ec2-user@$IP_ADDRESS:/home/ec2-user/

scp -i "~/.ssh/keys/remotebuild.pem" \
-o StrictHostKeyChecking=no \
../Dockerfile.x86 \
ec2-user@$IP_ADDRESS:/home/ec2-user/

USERNAME=$1
PASSWORD=$2

ssh -i "~/.ssh/keys/remotebuild.pem" \
-o StrictHostKeyChecking=no -t \
ec2-user@$IP_ADDRESS << EOF
    until [ -f ./output.txt ]
    do
        echo "waiting for file"
        sleep 3
    done
    echo "File found"
    sudo yum install docker -y
    sudo usermod -a -G docker ec2-user
    sudo systemctl start docker.service
    sudo docker login -u $USERNAME -p $PASSWORD
    sudo docker build . -f Dockerfile.x86 -t compute-unit
    sudo docker tag compute-unit $USERNAME/compute-unit:latest
    sudo docker push $USERNAME/compute-unit:latest
    cd ingress/frontend && sudo docker build . -t front-unit
    sudo docker tag front-unit $USERNAME/front-unit:latest
    sudo docker push $USERNAME/front-unit:latest
    cd ../../nanoservices/user-session-cache/cache-module 
    sudo docker build . -t redis-cache
    sudo docker tag redis-cache $USERNAME/redis-cache:latest
    sudo docker push $USERNAME/redis-cache:latest
EOF
