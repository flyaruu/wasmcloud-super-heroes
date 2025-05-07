#!/bin/sh
cargo build --target wasm32-wasip2 --release
wash start provider ghcr.io/wasmcloud/sqldb-postgres:0.9.0 postgres-provider --ctl-host nats
set -e
cd services/rest-heroes
./build.sh
cd ..
cd rest-villains
./build.sh
cd ..
cd rest-locations
./build.sh
cd ..
cd rest-fights
./build.sh
cd ../..

# curl wasmcloud:8000/api/fights/randomfight