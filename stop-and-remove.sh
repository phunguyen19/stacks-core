#!/bin/bash
set -e

# Stop the container if it's running
if podman ps -a | grep -q api-proxy; then
  echo "Stopping api-proxy container..."
  podman stop api-proxy || true
  echo "Removing api-proxy container..."
  podman rm api-proxy || true
else
  echo "No api-proxy container found."
fi
