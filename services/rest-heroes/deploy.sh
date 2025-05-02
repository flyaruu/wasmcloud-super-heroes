#!/bin/sh
set -e
export SERVICE_NAME=rest-heroes
export BINARY_NAME=rest_heroes
echo "Deploying $SERVICE_NAME"

echo "Deploying $SERVICE_NAME"
wash build
wash push --insecure localhost:5001/superheroes/${SERVICE_NAME}:0.1.2 build/${BINARY_NAME}_s.wasm
wash app deploy --replace wadm.yaml