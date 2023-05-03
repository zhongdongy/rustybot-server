FROM rust:1.69.0-alpine3.17 AS builder

USER root
WORKDIR /app
COPY rustybot-server rustybot-server
COPY rustybot-macros rustybot-macros
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN apk add --no-cache musl-dev
RUN cargo build --package rustybot-server --release

FROM alpine:3.17
COPY --from=builder /app/target/release/rustybot-server /app/rustybot-server
WORKDIR /app
COPY log4rs.yml log4rs.yml
COPY version.yml version.yml
EXPOSE 9090
USER root

CMD ["/app/rustybot-server"]
