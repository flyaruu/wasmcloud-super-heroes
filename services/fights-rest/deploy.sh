#!/bin/sh
set -e
export SERVICE_NAME=fights-rest
export BINARY_NAME=fights_rest
echo "Deploying $SERVICE_NAME"
echo "Current directory: $PWD"
wash build
wash push --insecure localhost:5001/superheroes/${SERVICE_NAME}:0.1.2 ${PWD}/build/${BINARY_NAME}_s.wasm
