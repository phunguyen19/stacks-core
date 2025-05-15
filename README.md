<p align="left">
  <a href="https://stacks.co">
    <img alt="Stacks" src="https://i.imgur.com/zzwnCnY.png" width="250" />
  </a>
</p>

# Stacks API Proxy

A lightweight proxy service for the Stacks blockchain API.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg?style=flat)](https://www.gnu.org/licenses/gpl-3.0)
[![Discord Chat](https://img.shields.io/discord/621759717756370964.svg)](https://stacks.chat)

## Features

- Proxies requests to a Stacks node API
- Provides additional endpoints for custom data retrieval
- Parses and transforms Nakamoto blocks

## Building and Running

### Option 1: Using Podman

#### Prerequisites

- [Podman](https://podman.io/getting-started/installation)

#### Building and Running with Scripts

You can use the provided scripts to build and run the API Proxy:

```bash
# Build and run the container
./build-and-run.sh

# Stop and remove the container
./stop-and-remove.sh
```

#### Building and Running Manually

```bash
# Build the image
podman build -t api-proxy:latest .

# Run the container
podman run -d --name api-proxy \
  -p 8080:8080 \
  -e RUST_LOG=info \
  -e BIND_ADDRESS=0.0.0.0:8080 \
  -e STACKS_NODE_URL=https://stacks-node-api.mainnet.stacks.co \
  api-proxy:latest
```

#### Using Docker Compose with Podman

```bash
# Run with docker-compose
podman-compose up -d

# Stop and remove containers
podman-compose down
```

### Option 2: Building Locally

#### 1. Download and install Rust

_For building on Windows, follow the rustup installer instructions at https://rustup.rs/._

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup component add rustfmt
```

#### 2. Build and run the API Proxy

```bash
# Build the api-proxy
cargo build -p api-proxy --release

# Run the api-proxy
cargo run -p api-proxy
```

## Configuration

The API Proxy can be configured using environment variables:

- `BIND_ADDRESS`: The address and port to bind to (default: `127.0.0.1:8080`)
- `STACKS_NODE_URL`: The URL of the Stacks node API (default: `https://stacks-node-api.mainnet.stacks.co`)
- `RUST_LOG`: The log level (default: `info`)

## API Endpoints

- `GET /v1/health`: Basic health check
- `GET /v1/health/detailed`: Detailed health check
- `GET /v1/blocks/height/{height}/txids`: Get transaction IDs for a block
- `GET /v1/proxy/v3/blocks/height/{height}`: Get Nakamoto block at a specific height

## Testing

**Run the tests:**

```bash
cargo test -p api-proxy
```

## Further Reading

You can learn more by visiting [the Stacks Website](https://stacks.co) and checking out the documentation:

- [Stacks docs](https://docs.stacks.co/)
- [RPC endpoints](https://docs.stacks.co/docs/api/)

## Copyright and License

The code and documentation copyright are attributed to stacks.org.

This code is released under the [GPL v3 license](https://www.gnu.org/licenses/quick-guide-gplv3.en.html), and the docs are released under the [Creative Commons license](https://creativecommons.org/).
