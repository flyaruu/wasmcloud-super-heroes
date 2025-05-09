#!/bin/sh
set -e
export SERVICE_NAME=rest-locations
export BINARY_NAME=rest_locations
wash config put locations-postgres \
  POSTGRES_HOST=locations-db \
  POSTGRES_PORT=5432 \
  POSTGRES_USERNAME=superman \
  POSTGRES_PASSWORD=superman \
  POSTGRES_DATABASE=locations_database \
  POSTGRES_TLS_REQUIRED=false
echo "Deploying $SERVICE_NAME"
wash build
wash push --insecure localhost:5001/superheroes/${SERVICE_NAME}:0.1.0 build/${BINARY_NAME}_s.wasm
wash app deploy --replace wadm.yaml