FROM rust:1.75-alpine AS builder

WORKDIR /app

RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

COPY . .

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/network-probe /usr/local/bin/network-probe

RUN chmod +x /usr/local/bin/network-probe

WORKDIR /app

EXPOSE 8080

ENTRYPOINT ["network-probe"]
CMD ["server", "--host", "0.0.0.0", "--port", "8080"]
