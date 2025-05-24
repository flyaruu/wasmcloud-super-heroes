#!/bin/sh
set -e
WASMCLOUD_NATS_HOST=${WASMCLOUD_NATS_HOST:-"localhost"}
export SERVICE_NAME=villain-repository
export BINARY_NAME=villain_repository
export WASM_REGISTRY_BASE_URL=${1:-'localhost:5001/superheroes'}
echo "Deploying $SERVICE_NAME"
wash config put villains-postgres --ctl-host ${WASMCLOUD_NATS_HOST}\
  POSTGRES_HOST=villains-db \
  POSTGRES_PORT=5432 \
  POSTGRES_USERNAME=superman \
  POSTGRES_PASSWORD=superman \
  POSTGRES_DATABASE=villains_database \
  POSTGRES_TLS_REQUIRED=false
wash build
wash push --insecure ${WASM_REGISTRY_BASE_URL}/${SERVICE_NAME}:0.1.0 build/${BINARY_NAME}_s.wasm
