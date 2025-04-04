FROM rust:latest

LABEL authors="juanpablohernandezmosti"

WORKDIR /app

COPY . /app

RUN cargo build

CMD ["cargo", "run"]