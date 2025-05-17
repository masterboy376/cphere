FROM rust:1.84 AS builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "// dummy" > src/lib.rs && cargo fetch

# copy your code
COPY src/ ./src/

# build the release binary
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /usr/src/app

RUN apt-get update \
 && apt-get install -y libssl3 ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# copy only the release binary (crate name = cphere-backend)
COPY --from=builder /usr/src/app/target/release/cphere-backend ./cphere-backend

EXPOSE 8080
CMD ["./cphere-backend"]
