# Stage 1: Build the application
FROM rust:1.81-bookworm as builder

# Set the working directory inside the container
WORKDIR /usr/src/friday-todo-manager

# Copy the source code files to the working directory
COPY src/ ./src/
COPY Cargo.lock .
COPY Cargo.toml .

# Install the application dependencies and build the application
RUN cargo install --path .

# Stage 2: Create the final image
FROM debian:bookworm

# Copy the .env.prod file to the working directory
COPY .env.prod .

# Copy the built application binary from the builder stage to the final image
COPY --from=builder /usr/local/cargo/bin/friday-todo-manager /usr/local/bin/friday-todo-manager

# Set the command to run when the container starts
CMD ["friday-todo-manager"]
