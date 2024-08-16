FROM rust:1.80-alpine AS builder

WORKDIR /app

RUN apk add alpine-sdk musl-dev build-base upx curl

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl
RUN upx --best --lzma /app/target/x86_64-unknown-linux-musl/release/bayen

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bayen /app

ENTRYPOINT ["/app"]
