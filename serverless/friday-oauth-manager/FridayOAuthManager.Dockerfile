# Stage 1: Build the application
FROM rust:1.88-bookworm AS builder

# Set the working directory inside the container
WORKDIR /usr/src/friday-oauth-manager

# Copy the source code files to the working directory
COPY src/ ./src/
COPY Cargo.lock .
COPY Cargo.toml .

# Install the application dependencies and build the application
RUN cargo install --path .

# Stage 2: Create the final image
FROM debian:bookworm

# Instalar dependências necessárias para o binário Rust
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the .env file to the working directory
COPY .env .

# Copy the built application binary from the builder stage to the final image
COPY --from=builder /usr/local/cargo/bin/friday-oauth-manager /usr/local/bin/friday-oauth-manager

# Set the command to run when the container starts
CMD ["friday-oauth-manager"]
