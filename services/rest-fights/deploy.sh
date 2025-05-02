#!/bin/sh
set -e
export SERVICE_NAME=rest-fights
export BINARY_NAME=rest_fights
echo "Deploying $SERVICE_NAME"
echo "Current directory: $PWD"
wash build
wash push --insecure localhost:5001/superheroes/${SERVICE_NAME}:0.1.0 ${PWD}/build/${BINARY_NAME}_s.wasm
wash app deploy --replace wadm.yaml