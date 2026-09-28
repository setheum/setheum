FROM golang:1.25-bookworm AS builder
WORKDIR /app
COPY repos/bridge/bridge-relayer/ .
RUN go mod download && go build -o /bridge .

FROM gcr.io/distroless/cc-debian12:latest
COPY --from=builder /bridge /usr/local/bin/bridge
COPY --from=alpine:latest /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
ENTRYPOINT ["bridge"]