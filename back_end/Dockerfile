# Use the Rust musl cross image as the base image
FROM rust:latest

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    gcc \
    make \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app
COPY . .
#
## Copy the Cargo.toml and source code
#COPY Cargo.toml Cargo.lock ./
#COPY src ./src
#
## Build the Rust project for musl target
RUN cargo build --release
#
## Expose the port your server will run on
EXPOSE 8080
#
## Set the command to run the application
CMD ["./target/release/server-template", "config.yml"]
