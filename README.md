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
```

### Push the wasm file to the internal OCI registry:
```bash
wash push --insecure localhost:5001/flyaruu/rest-heroes:0.1.2 /Users/flyaruu/git/wasmcloud-super-heroes/services/rest-heroes/build/rest_heroes_s.wasm
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
## Running with wasmtime

You must have wasmtime >=25.0.0 for this to work. Make sure to follow the build step above first.

```bash
wasmtime serve -Scommon ./build/dog_fetcher_s.wasm
```

## Running with wasmCloud

Ensuring you've built your component with `wash build`, you can launch wasmCloud and deploy the full
hello world application with the following commands. Once the application reports as **Deployed** in
the application list, you can use `curl` to send a request to the running HTTP server.

```shell
wash up -d
wash app deploy ./wadm.yaml
wash app get
curl http://127.0.0.1:8000
```

## Adding Capabilities

To learn how to extend this example with additional capabilities, see the [Adding
Capabilities](https://wasmcloud.com/docs/tour/adding-capabilities?lang=rust) section of the
wasmCloud documentation.
