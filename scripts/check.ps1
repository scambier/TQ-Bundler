Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Host "Running tests..."
cargo test --all-targets --locked

Write-Host "Building release binary..."
cargo build --release --locked

Write-Host "Checks passed."
