# build container
FROM rust:1.85.0-slim-bookworm AS backend
RUN apt update && apt install -y librust-openssl-dev libssl-dev
RUN mkdir /app
COPY rust /app/rust
COPY artifacts /app/artifacts
RUN cd /app/rust && cargo build --release

# target container
FROM rust:1.85.0-slim-bookworm
RUN mkdir /app
COPY --from=backend /app/rust/target/release/rust /app/rust
COPY --from=backend /app/artifacts /app/artifacts
WORKDIR /app
CMD ["./app/rust/target/release/rust"]
EXPOSE 8080
ENV RUST_LOG="info"