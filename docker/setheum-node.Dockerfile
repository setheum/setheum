FROM rust:1.88-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y git clang && rm -rf /var/lib/apt/lists/*
COPY . .
RUN git submodule update --init --recursive
RUN SKIP_WASM_BUILD=1 cargo build --release -p setheum-node && \
    cp target/release/setheum-node /usr/local/bin/setheum-node

FROM gcr.io/distroless/cc-debian12:latest
COPY --from=builder /usr/local/bin/setheum-node /usr/local/bin/setheum-node
EXPOSE 30333 30343 9944
ENTRYPOINT ["setheum-node"]