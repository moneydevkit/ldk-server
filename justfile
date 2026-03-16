default:
    @just --list

# Run all checks (fmt, clippy, test, test-all-features)
check: fmt-check clippy test test-all

# Format code
fmt:
    cargo fmt --all
    nixfmt flake.nix

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check
    nixfmt --check flake.nix

# Auto-fix lint issues
fix:
    cargo clippy --all-features --fix --allow-dirty --allow-staged

# Run clippy check
clippy:
    cargo clippy --all-features -- -D warnings -A clippy::drop_non_drop

# Run tests
test:
    cargo test

# Run tests with all features
test-all:
    just _with-rabbitmq cargo test --all-features

# Run e2e tests
test-e2e *args:
    just _with-rabbitmq cargo test --manifest-path e2e-tests/Cargo.toml -- --test-threads=4 {{args}}

# Run the server
run *args:
    cargo run --bin ldk-server {{args}}

# Run the CLI
cli *args:
    cargo run --bin ldk-server-cli -- {{args}}

# Generate protocol buffers
proto:
    RUSTFLAGS="--cfg genproto" cargo build -p ldk-server-protos
    cargo fmt --all

# Clean build artifacts
clean:
    cargo clean

[private]
_with-rabbitmq +cmd:
    #!/usr/bin/env bash
    set -euo pipefail
    export RABBITMQ_MNESIA_BASE=$(mktemp -d)
    export RABBITMQ_LOG_BASE=$(mktemp -d)
    cleanup() { rabbitmqctl stop 2>/dev/null || true; rm -rf "$RABBITMQ_MNESIA_BASE" "$RABBITMQ_LOG_BASE"; }
    trap cleanup EXIT
    rabbitmq-server &
    rabbitmqctl await_startup --timeout 30
    {{cmd}}
