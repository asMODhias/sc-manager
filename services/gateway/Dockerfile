FROM rust:latest as builder
WORKDIR /usr/src
# Copy the whole repository so path dependencies are available
COPY . .
# Ensure Cargo is up to date (supports newer lockfile versions) then build only the gateway package
RUN cargo --version && cargo build --release -p sc_manager_gateway --features run_server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/target/release/sc_manager_gateway /usr/local/bin/sc_manager_gateway
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/sc_manager_gateway"]
