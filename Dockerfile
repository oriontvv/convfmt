# convfmt is a stdin -> stdout filter, so the image only needs the binary itself.
# rust:alpine targets musl by default, which gives a fully static binary for scratch.
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /src

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked && strip target/release/convfmt

FROM scratch

LABEL org.opencontainers.image.title="convfmt" \
      org.opencontainers.image.description="cli tool which can convert different formats" \
      org.opencontainers.image.source="https://github.com/oriontvv/convfmt" \
      org.opencontainers.image.licenses="Apache-2.0"

COPY --from=builder /src/target/release/convfmt /convfmt

ENTRYPOINT ["/convfmt"]
