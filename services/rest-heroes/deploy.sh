#!/bin/sh
set -e
export SERVICE_NAME=rest-heroes
export BINARY_NAME=rest_heroes
echo "Deploying $SERVICE_NAME"
wash config put heroes-postgres \
  POSTGRES_HOST=heroes-db \
  POSTGRES_PORT=5432 \
  POSTGRES_USERNAME=superman \
  POSTGRES_PASSWORD=superman \
  POSTGRES_DATABASE=heroes_database \
  POSTGRES_TLS_REQUIRED=false
echo "Deploying $SERVICE_NAME"
wash build
wash push --insecure localhost:5001/superheroes/${SERVICE_NAME}:0.1.2 build/${BINARY_NAME}_s.wasm
wash app deploy --replace wadm.yaml