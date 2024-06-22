FROM rust:1.79.0-alpine AS build
ARG TARGET=x86_64-unknown-linux-musl
ARG PROJECT=rook-action

RUN apk add musl-dev upx
RUN update-ca-certificates

RUN rustup update nightly && rustup default nightly && \
    rustup component add rust-src --toolchain nightly

RUN \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=src,target=src \
    cargo +nightly build \
        -Z build-std=std,panic_abort \
        -Z build-std-features=panic_immediate_abort \
        --target ${TARGET} \
        --release
RUN upx --best --lzma target/${TARGET}/release/${PROJECT}
RUN cp target/${TARGET}/release/${PROJECT} /entrypoint

FROM scratch
COPY --from=build /entrypoint /
USER 1000
ENTRYPOINT ["/entrypoint"]