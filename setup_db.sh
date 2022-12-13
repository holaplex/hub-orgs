#! /bin/bash

source .env.dev
DATABASE_NAME=$DATABASE_NAME 
DATABASE_PASSWORRD=$DATABASE_PASSWORD

export DATABASE_NAME 
export DATABASE_PASSWORD


# run docker-compose and check connection
docker-compose up -d 
echo "checking postgres connection.."

CONTAINER_NAME="hub_orgs_dev_db"

for i in {1..5}; do
    if docker exec $CONTAINER_NAME pg_isready 
    then break
    fi
    sleep 3;
done
