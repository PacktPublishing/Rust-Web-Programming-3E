#!/usr/bin/env bash
# deployment/run_deployment.sh

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

USERNAME=$1
PASSWORD=$2
DB_USERNAME=$3
DB_PASSWORD=$4

terraform init

terraform apply \
-var="db_username=$DB_USERNAME" \
-var="db_password=$DB_PASSWORD" \
-auto-approve

terraform output -json > ./output.json

OUTPUT_FILE="output.json"

IP_ADDRESS=$(jq --raw-output '.ec2_global_ips.value[0][0]' \
$OUTPUT_FILE)
# IP_ADDRESS_TWO=$(jq --raw-output '.ec2_global_ips.value[0][1]' \
# $OUTPUT_FILE)
# IP_ADDRESSES=("$IP_ADDRESS" "$IP_ADDRESS_TWO")
IP_ADDRESSES=("$IP_ADDRESS")
echo $IP_ADDRESS
# echo $IP_ADDRESS_TWO
echo "All IP Addresses: ${IP_ADDRESSES[@]}"
DB_ENDPOINT=$(jq -r '.db_endpoint.value[0]' $OUTPUT_FILE)

DB_PORT=5432
DB_NAME="to_do"
POSTGRES_URL="postgresql://${DB_USERNAME}:${DB_PASSWORD}"\
"@${DB_ENDPOINT}/${DB_NAME}"
echo "PostgreSQL URL: $POSTGRES_URL"

for IP in "${IP_ADDRESSES[@]}"
do
  echo "Processing IP: $IP"
  scp -i "~/.ssh/keys/remotebuild.pem" \
  -o StrictHostKeyChecking=no \
  ./docker-compose.yml \
  ec2-user@$IP:/home/ec2-user/

  scp -i "~/.ssh/keys/remotebuild.pem" \
  -o StrictHostKeyChecking=no \
  ./nginx.conf \
  ec2-user@$IP:/home/ec2-user/

  ssh -i "~/.ssh/keys/remotebuild.pem" \
  -o StrictHostKeyChecking=no -t \
  
  ssh -i "~/.ssh/keys/remotebuild.pem" \
  -o StrictHostKeyChecking=no -t \
  ec2-user@$IP << EOF
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
      export DATABASE_URL=$POSTGRES_URL
      sudo DATABASE_URL=$POSTGRES_URL docker-compose up -d
  EOF
done

curl -X POST https://pubsub.me/api/v1/users/create \
-H "Content-Type: application/json" \
-d '{
  "email": "test@gmail.com",
  "password": "password"
}'

# curl -u test@gmail.com:password -X GET \
# http://$IP_ADDRESS/api/v1/auth/login
