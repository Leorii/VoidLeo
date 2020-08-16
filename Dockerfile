FROM rust:1.45-slim-buster AS build
WORKDIR /usr/src

RUN USER=root cargo new voidleo
WORKDIR /usr/src/voidleo
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs
RUN rm target/release/deps/voidleo*

COPY src ./src
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /usr/src/voidleo/target/release/voidleo /
COPY config.ron ./

USER 1000
ENV RUST_LOG=voidleo=info

CMD ["./voidleo"]
