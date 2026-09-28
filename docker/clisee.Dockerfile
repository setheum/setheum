FROM rust:1.88-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y git clang && rm -rf /var/lib/apt/lists/*
COPY . .
RUN git submodule update --init --recursive
RUN SKIP_WASM_BUILD=1 cargo build --release -p clisee && \
    cp target/release/clisee /usr/local/bin/clisee

FROM gcr.io/distroless/cc-debian12:latest
COPY --from=builder /usr/local/bin/clisee /usr/local/bin/clisee
ENTRYPOINT ["clisee"]