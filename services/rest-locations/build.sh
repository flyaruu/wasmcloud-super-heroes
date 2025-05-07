#!/bin/sh
# This script builds the service and deploys it to the NATS server.
# It only works within a the builder container.
set -e
export SERVICE_NAME=rest-locations
export BINARY_NAME=rest_locations
echo "Deploying $SERVICE_NAME"
wash config put --ctl-host nats locations-postgres \
  POSTGRES_HOST=locations-db \
  POSTGRES_PORT=5432 \
  POSTGRES_USERNAME=superman \
  POSTGRES_PASSWORD=superman \
  POSTGRES_DATABASE=locations_database \
  POSTGRES_TLS_REQUIRED=false
echo "Deploying $SERVICE_NAME"
mkdir -p build
cp ../../target/wasm32-wasip2/release/${BINARY_NAME}.wasm build/${BINARY_NAME}.wasm
wash build --sign-only
wash push --insecure registry:5000/superheroes/${SERVICE_NAME}:0.1.0 build/${BINARY_NAME}_s.wasm
wash app deploy --ctl-host nats --replace wadm.yaml