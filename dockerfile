#  Building the application
FROM rust:1.71 as builder

# Clone the repository
RUN git clone https://github.com/why-arong/simple_web_server_rust.git /simple_web_server
WORKDIR /simple_web_server

# Build the application for release
RUN cargo build --release

# Setup the runtime environmentd
FROM ubuntu:22.04

# Copy the build artifact from the build stage
COPY --from=builder /simple_web_server/target/release/simple_web_server /usr/local/bin/simple_web_server

# Set the ENTRYPOINT to run the web server
ENTRYPOINT ["/usr/local/bin/simple_web_server"]

# Expose the port the server is running on
EXPOSE 8080
