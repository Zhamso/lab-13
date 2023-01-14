FROM rust:1.64-slim-bullseye
WORKDIR /usr/src/
COPY . .
# COPY ./target /usr/src/todos/target
EXPOSE 80:3000
RUN cargo install --path .

CMD ["cargo", "run"]