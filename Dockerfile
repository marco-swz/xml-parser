FROM rust:1.82.0-slim

COPY . /app

WORKDIR /app

RUN cargo build --release

