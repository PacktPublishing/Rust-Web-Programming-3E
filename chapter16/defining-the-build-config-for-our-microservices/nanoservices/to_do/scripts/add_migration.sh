#!/usr/bin/env bash

# Check if argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <migration-name>"
    exit 1
fi

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH/../dal/migrations

current_timestamp=$(date +'%Y%m%d%H%M%S')
description="$1"
script_name="${current_timestamp}_${description}.sql"
touch $script_name

echo "Creating migration script: $script_name"