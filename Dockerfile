FROM rust:1.96 AS builder
WORKDIR /app
COPY . .
RUN rustc main.rs -o rust-webserver

FROM debian:13-slim
WORKDIR /app
COPY --from=builder /app/rust-webserver /app/rust-webserver
CMD ["/app/rust-webserver"]
