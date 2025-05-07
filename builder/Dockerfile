# FROM ubuntu:latest
FROM rust:1.85
# Install dependencies
WORKDIR /build
RUN apt-get update && apt-get install -y \
    curl \
    unzip && \
    curl -s https://packagecloud.io/install/repositories/wasmcloud/core/script.deb.sh | bash && \
    apt-get install wash && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-wasip2
# Verify installation
RUN wash --version
RUN cargo build --target wasm32-wasip2 --release

CMD ["/bin/bash"]