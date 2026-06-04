FROM rust:1.96

WORKDIR /app/

COPY . .

RUN rustc main.rs
CMD ["/app/main"]
