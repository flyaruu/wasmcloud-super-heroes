#!/bin/sh
set -e
WASMCLOUD_NATS_HOST=${WASMCLOUD_NATS_HOST:-"localhost"}
export SERVICE_NAME=fights-rest
export BINARY_NAME=fights_rest
export WASM_REGISTRY_BASE_URL=${1:-'localhost:5001/superheroes'}
echo "Deploying $SERVICE_NAME"
echo "Current directory: $PWD"
wash build
wash push --insecure ${WASM_REGISTRY_BASE_URL}/${SERVICE_NAME}:0.1.2 ${PWD}/build/${BINARY_NAME}_s.wasm
echo "Deployment of $SERVICE_NAME completed successfully."