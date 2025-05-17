#!/bin/sh
set -e
export SERVICE_NAME=location-rest
export BINARY_NAME=location_rest
echo "Deploying $SERVICE_NAME"
wash build
wash push --insecure localhost:5001/superheroes/${SERVICE_NAME}:0.1.0 build/${BINARY_NAME}_s.wasm
