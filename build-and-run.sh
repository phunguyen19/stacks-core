#!/bin/bash
set -e

# Build the container image
echo "Building api-proxy container image..."
podman build -t api-proxy:latest .

# Run the container
echo "Running api-proxy container..."
podman run -d --name api-proxy \
  -p 8080:8080 \
  -e RUST_LOG=info \
  -e BIND_ADDRESS=0.0.0.0:8080 \
  -e STACKS_NODE_URL=https://stacks-node-api.mainnet.stacks.co \
  api-proxy:latest

echo "API Proxy is running at http://localhost:8080"
