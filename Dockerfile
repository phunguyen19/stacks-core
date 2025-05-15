FROM rust:bookworm AS build

WORKDIR /src
COPY . .
RUN mkdir /out
RUN rustup toolchain install stable
RUN cargo build -p api-proxy
RUN cp -R target/debug/api-proxy /out/

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=build /out/api-proxy /bin/

# Set environment variables
ENV RUST_LOG=info
ENV BIND_ADDRESS=0.0.0.0:8080
ENV STACKS_NODE_URL=https://stacks-node-api.mainnet.stacks.co

# Expose the port
EXPOSE 8080

# Run the api-proxy
CMD ["api-proxy"]
