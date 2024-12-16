FROM rust:1.83-alpine AS builder
WORKDIR /usr/src/infinitystreamer
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY assets assets
RUN apk add musl-dev
RUN cargo install --path . --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/local/cargo/bin/infinitystreamer /usr/local/bin/infinitystreamer
ENV STREAM_HOST=::
ENV STREAM_PORT=80
EXPOSE 80/tcp
CMD ["infinitystreamer"]
