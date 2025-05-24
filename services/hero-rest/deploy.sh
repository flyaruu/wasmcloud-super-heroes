#!/bin/sh
set -e
set -x
WASMCLOUD_NATS_HOST=${WASMCLOUD_NATS_HOST:-"localhost"}
export SERVICE_NAME=hero-rest
export BINARY_NAME=hero_rest
export WASM_REGISTRY_BASE_URL=${1:-'localhost:5001/superheroes'}
wash build
wash push --insecure ${WASM_REGISTRY_BASE_URL}/${SERVICE_NAME}:0.1.0 build/${BINARY_NAME}_s.wasm
