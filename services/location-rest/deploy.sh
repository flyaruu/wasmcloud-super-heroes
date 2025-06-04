#!/bin/sh
set -e -x
export SERVICE_NAME=location-rest
export BINARY_NAME=location_rest
WASMCLOUD_NATS_HOST=${WASMCLOUD_NATS_HOST:-"localhost"}
export WASM_REGISTRY_BASE_URL=${1:-'localhost:5001/superheroes'}

echo "Deploying $SERVICE_NAME"
wash build
wash push --insecure ${WASM_REGISTRY_BASE_URL}/${SERVICE_NAME}:0.1.0 build/${BINARY_NAME}_s.wasm
