#!/bin/sh
set -e
export SERVICE_NAME=villain-repository
export BINARY_NAME=villain_repository
echo "Deploying $SERVICE_NAME"
wash config put villains-postgres \
  POSTGRES_HOST=villains-db \
  POSTGRES_PORT=5432 \
  POSTGRES_USERNAME=superman \
  POSTGRES_PASSWORD=superman \
  POSTGRES_DATABASE=villains_database \
  POSTGRES_TLS_REQUIRED=false
wash build
wash push --insecure localhost:5001/superheroes/${SERVICE_NAME}:0.1.0 build/${BINARY_NAME}_s.wasm
