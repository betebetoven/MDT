# ---- Build Stage ----
FROM rust:1.70 as builder
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# Build the application
RUN cargo build --release

# ---- Runtime Stage ----
FROM rust:1.70
RUN apt-get update \
    && apt-get install -y --no-install-recommends ffmpeg \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the build stage
COPY --from=builder /app/target/release/mdt /usr/local/bin

CMD ["mdt"]