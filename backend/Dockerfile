FROM rust:1.65.0-slim-bullseye as builder

WORKDIR /usr/src/axum_tokio_app
COPY . .

RUN cargo install --path .

EXPOSE 80:3000

CMD ["axum_tokio_app"]