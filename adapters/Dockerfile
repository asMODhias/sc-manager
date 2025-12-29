FROM rust:latest

WORKDIR /usr/src/sc_manager_adapters

COPY . .

# Ensure Cargo is up-to-date so it can parse newer Cargo.lock formats
RUN rustup update stable

CMD ["cargo", "test", "--verbose"]
