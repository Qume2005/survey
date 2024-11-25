FROM rust:alpine AS builder
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
RUN apk add --no-cache build-base
COPY src/ src/
RUN cargo build --release
FROM alpine:latest
RUN apk add --no-cache libgcc
WORKDIR /app
COPY --from=builder /build/target/release/survey /app/survey
EXPOSE 3000
CMD ["/app/survey"]
