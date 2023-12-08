#  Building the application
FROM rust:1.71 as builder

# Clone the repository
RUN git clone https://github.com/why-arong/simple_web_server_rust.git /webserver
WORKDIR /webserver

# Build the application for release
RUN cargo build --release

# Setup the runtime environment
FROM ubuntu:22.04

# Copy the build artifact from the build stage
COPY --from=builder /webserver/target/release/webserver /usr/local/bin

# Set the ENTRYPOINT to run the web server
ENTRYPOINT ["webserver"]

# Expose the port the server is running on
EXPOSE 8080
