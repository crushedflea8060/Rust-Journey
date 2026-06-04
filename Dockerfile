FROM rust:1.96

WORKDIR /app/

COPY ./src/main.rs .
COPY . .

RUN rustc main.rs
RUN rm -rf ./src
CMD ["/app/main"]
