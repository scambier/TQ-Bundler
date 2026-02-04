#!/usr/bin/env bash
set -euo pipefail

echo "Running tests..."
cargo test --all-targets --locked

echo "Building release binary..."
cargo build --release --locked

echo "Checks passed."
