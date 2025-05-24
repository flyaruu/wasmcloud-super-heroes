#!/bin/sh
set -e
export SERVICE_NAME=location-rest
export BINARY_NAME=location_rest
WASMCLOUD_NATS_HOST=${WASMCLOUD_NATS_HOST:-"localhost"}

echo "Deploying $SERVICE_NAME"
wash build
wash push --insecure ${WASM_REGISTRY_BASE_URL}/${SERVICE_NAME}:0.1.0 build/${BINARY_NAME}_s.wasm
