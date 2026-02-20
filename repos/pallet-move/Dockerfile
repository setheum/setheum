# syntax=docker/dockerfile:1
FROM ubuntu:22.04 as builder
WORKDIR /root
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y build-essential make gcc g++ curl clang git libssl-dev pkg-config protobuf-compiler
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
WORKDIR /usr/src/move
RUN cargo install --git https://github.com/eigerco/smove
RUN git clone https://github.com/eigerco/pallet-move.git
RUN git clone https://github.com/eigerco/substrate-node-template-move-vm-test.git --branch pallet-move
RUN cd substrate-node-template-move-vm-test && cargo b -r

FROM ubuntu:22.04
RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/move/substrate-node-template-move-vm-test/target/release/node-template /usr/local/bin/node-template
EXPOSE 9333 9944 30333
CMD ["node-template", "--dev"]
