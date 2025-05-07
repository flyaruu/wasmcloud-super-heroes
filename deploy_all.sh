#!/bin/sh
set -e
wash start provider ghcr.io/wasmcloud/sqldb-postgres:0.9.0 postgres-provider
cd services/rest-heroes
./deploy.sh
cd ../..
cd services/rest-villains
./deploy.sh
cd ../..
cd services/rest-locations
./deploy.sh
cd ../..
cd services/rest-fights
./deploy.sh
cd ../..

