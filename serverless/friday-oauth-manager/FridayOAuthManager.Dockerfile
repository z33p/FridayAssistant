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

# Copy the .env file to the working directory
COPY .env .

# Copy the built application binary from the builder stage to the final image
COPY --from=builder /usr/local/cargo/bin/friday-oauth-manager /usr/local/bin/friday-oauth-manager

# Expose port 3000
EXPOSE 3000

# Set the command to run when the container starts
CMD ["friday-oauth-manager"]
