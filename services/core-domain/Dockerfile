FROM rust:latest

WORKDIR /usr/src/sc_manager_core

# Install typical build deps for Rust projects
RUN apt-get update -qq && \
    apt-get install -y -qq build-essential pkg-config libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy project
COPY . /usr/src/sc_manager_core

# Prebuild to cache dependencies
RUN cargo build --release || true

# Default command is overridden by docker-compose (cargo test)
