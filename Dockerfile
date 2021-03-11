
# Build Stage
FROM rust:1.49.0 AS builder
RUN rustup target add x86_64-unknown-linux-musl

# Build cargo dependancies in a dummy project for caching purposes
RUN USER=root cargo new proj
WORKDIR /proj
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --target x86_64-unknown-linux-musl --release --features include_site
RUN rm src/*.rs

# Build the whole app
COPY ./src ./src/
# For when we bundle assets with the binary, we'll want to copy the web distrobution for compile time static file inclusion
# COPY ./dist ./dist/
RUN cargo build --target x86_64-unknown-linux-musl --release --features include_site
RUN mv /proj/target/x86_64-unknown-linux-musl/release/duckception .
RUN strip duckception

# Bundle Stage
FROM scratch
COPY --from=builder /proj/duckception .
COPY ./dist ./dist/
USER 1000
ENTRYPOINT ["./duckception", "$PORT"]
