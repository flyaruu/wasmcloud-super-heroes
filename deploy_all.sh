#!/bin/sh
WASMCLOUD_NATS_HOST=${WASMCLOUD_NATS_HOST:-"localhost"}

wash start provider --ctl-host ${WASMCLOUD_NATS_HOST} ghcr.io/wasmcloud/sqldb-postgres:0.9.0 postgres-provider

set -e
cargo build --release
for dir in services/*/; do
  if [ -x "$dir/deploy.sh" ]; then
    echo "Deploying service in $dir"
    (cd "$dir" && ./deploy.sh ${WASM_REGISTRY_BASE_URL})
  fi
done
echo "Deploying wadm to host ${WASMCLOUD_NATS_HOST}"
wash app deploy --ctl-host ${WASMCLOUD_NATS_HOST} --replace ./services/wadm.yaml