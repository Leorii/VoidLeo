FROM rust:1.45.0 AS build
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new leo-bot
WORKDIR /usr/src/leo-bot
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/leo-bot .
USER 1000
CMD ["./leo-bot"]
