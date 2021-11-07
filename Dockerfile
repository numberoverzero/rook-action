FROM rust:1.56.1 AS build
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new rook-action
WORKDIR /usr/src/rook-action
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/rook-action .
USER 1000
ENTRYPOINT ["./rook-action"]