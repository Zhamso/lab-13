FROM rust:1.64-slim-bullseye as builder
WORKDIR /usr/src/todos
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/todos /usr/local/bin/todos
CMD ["todos"]