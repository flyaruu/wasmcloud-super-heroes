# Rust Superheroes, wasmCloud edition

## Prerequisites

- `cargo` 1.75
- [`wash`](https://wasmcloud.com/docs/installation) 0.27.0
- `wasmtime` >=25.0.0 (if running with wasmtime)

## Building / Running

For the (rest-heroes service)
First spin up the infrastructure
```bash
docker compose up
```
... this will start:
- The heroes database (Postgres)
- The villains database (Postgres)
- The locations database (MariaDB)
- a wasmCloud host
- a wadm instance
- a single node Nats cluster

### Build the webassembly apps:
```bash
cd services/rest-heroes
wash build
cd ../..
```

### Push the wasm file to the internal OCI registry:
```bash
wash push --insecure localhost:5001/flyaruu/rest-heroes:0.1.2 services/rest-heroes/build/rest_heroes_s.wasm
```

### Install the Postgres 'driver' (Capability provider)
```bash
wash start provider ghcr.io/wasmcloud/sqldb-postgres:0.9.0 postgres-provider
```

### Deploy the wadm file:
```bash
wash app deploy --replace ./wadm.yaml
```

### Configure the database:
```bash
wash config put default-postgres \
  POSTGRES_HOST=heroes-db \
  POSTGRES_PORT=5432 \
  POSTGRES_USERNAME=superman \
  POSTGRES_PASSWORD=superman \
  POSTGRES_DATABASE=heroes_database \
  POSTGRES_TLS_REQUIRED=false

```
### Call the service using cUrl
```bash
curl -v localhost:8000/api/heroes
```
... will return a list of all heroes in JSON

