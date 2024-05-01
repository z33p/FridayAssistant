# Use the official Rust image as the base image
FROM rust:1.75 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY src/ ./src/
COPY Cargo.lock .
COPY Cargo.toml .

# Build the dependencies
RUN cargo build --release

# Copy the source code to the container
COPY src ./src

# Build the application
RUN cargo build --release

# Create a new stage for the final image
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage to the final image
COPY --from=builder /app/target/release/friday-secret-manager .

# Expose the port(s) that the application listens on
EXPOSE 8000

# Set the command to run the application
CMD ["./friday-secret-manager"]