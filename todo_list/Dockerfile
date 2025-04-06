FROM rust:latest

LABEL authors="juanpablohernandezmosti"

WORKDIR /app

COPY Cargo.toml /app

COPY Cargo.lock /app

COPY . /app

RUN cargo build

CMD ["cargo", "run"]