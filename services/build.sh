#!/bin/sh
wash start provider ghcr.io/wasmcloud/sqldb-postgres:0.9.0 postgres-provider --ctl-host nats
set -e
cd rest-heroes
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
cd ..

# curl wasmcloud:8000/api/fights/randomfight