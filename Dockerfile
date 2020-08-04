FROM rust:1.45-slim-buster AS build
WORKDIR /usr/src

RUN USER=root cargo new void-leo
WORKDIR /usr/src/void-leo
COPY src ./src
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

FROM slim-buster
COPY --from=build /usr/src/void-leo/target/release/void-leo/
USER 1000
CMD ["./void-leo"]
