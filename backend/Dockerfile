# Build stage
FROM rust:latest as builder

# Set the working directory for subsequent instructions
WORKDIR /backend

# Copy the current directory (including the code) into the container
COPY . .

# Build and install the Rust application using Cargo
RUN cargo install --path .

# Final stage
FROM debian:bullseye-slim

# Update the package list and install extra runtime dependencies
RUN apt-get update && apt-get install -y protobuf-compiler postgresql postgresql-client && rm -rf /var/lib/apt/lists/*

# Copy the built executable from the previous stage to the final image
COPY --from=builder /usr/local/cargo/bin/versequest-be /usr/local/bin/versequest-be

# Set the command to be executed when the container starts
CMD ["versequest-be"]
