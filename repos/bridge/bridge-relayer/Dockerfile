# Copyright 2020 ChainSafe Systems
# SPDX-License-Identifier: LGPL-3.0-only

FROM alpine:3.6 as alpine
RUN apk add -U --no-cache ca-certificates

FROM  golang:1.19 AS builder
ADD . /src
WORKDIR /src
RUN cd /src && echo $(ls -1 /src)
RUN go mod download
RUN go build -ldflags "-X google.golang.org/protobuf/reflect/protoregistry.conflictPolicy=ignore -X github.com/ChainSafe/sygma-relayer/app.Version=$(sed -n '0,/## \[\([0-9.]*\)\]/s/.*\[\([0-9.]*\)\].*/\1/p' CHANGELOG.md)" -o /bridge .

# final stage
FROM debian:stable-slim
COPY --from=builder /bridge ./
RUN chmod +x ./bridge
RUN mkdir -p /mount
COPY --from=alpine /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENTRYPOINT ["./bridge"]
