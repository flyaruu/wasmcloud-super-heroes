#!/bin/sh
wash start provider ghcr.io/wasmcloud/sqldb-postgres:0.9.0 postgres-provider
set -e

for dir in services/*/; do
  if [ -x "$dir/deploy.sh" ]; then
    (cd "$dir" && ./deploy.sh)
  fi
done

wash app deploy --replace ./services/wadm.yaml