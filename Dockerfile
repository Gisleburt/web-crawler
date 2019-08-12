FROM rust:buster as builder

RUN apt-get update \
 && apt-get install -y \
    musl-tools \
    pkg-config \
    libssl-dev

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl --features vendored

FROM alpine

RUN apk add --no-cache ca-certificates

WORKDIR /app

ENV PORT=80
ENV PRODUCTION=1

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/crawl-api /app

CMD ["/app/crawl-api"]
