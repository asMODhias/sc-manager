FROM rust:latest

WORKDIR /usr/src/sc_manager_core

COPY . .

# Ensure Cargo is up-to-date so it can parse newer Cargo.lock formats
RUN rustup update stable

# Run tests at build to fail fast in CI
CMD ["cargo", "test", "--verbose"]
