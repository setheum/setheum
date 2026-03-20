FROM ubuntu:jammy-20220531

RUN apt update && \
    apt install curl -y && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

EXPOSE 30333 30343 9944

WORKDIR /node

COPY target/release/setheum-node /usr/local/bin
RUN chmod +x /usr/local/bin/setheum-node

COPY docker/docker_entrypoint.sh /node/docker_entrypoint.sh
RUN chmod +x /node/docker_entrypoint.sh

ENTRYPOINT ["./docker_entrypoint.sh"]
