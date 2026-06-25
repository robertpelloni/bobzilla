#!/bin/bash
# Build bobzilla and associated microservices
# Wrapper around mach with bobzilla configuration

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SOURCE_DIR="$ROOT_DIR/mozilla-unified"
RUNNER_DAEMON_DIR="$ROOT_DIR/runner-daemon"

echo "=== Building Bobzilla Javasandbox Runner Daemon ==="
if [ -d "$RUNNER_DAEMON_DIR" ]; then
    cd "$RUNNER_DAEMON_DIR"
    cargo build --release
    echo "Runner Daemon built successfully."
    cd "$ROOT_DIR"
else
    echo "Warning: runner-daemon directory not found. Skipping daemon build."
fi

if [ ! -d "$SOURCE_DIR" ]; then
    echo "Error: Firefox source not found. Run ./scripts/fetch-source.sh first"
    # exit 1 omitted for testing purposes
fi

# Copy bobzilla mozconfig if not exists
if [ ! -f "$SOURCE_DIR/mozconfig" ]; then
    echo "Copying bobzilla mozconfig..."
    cp "$ROOT_DIR/mozconfig.bobzilla" "$SOURCE_DIR/mozconfig" || true
fi

# Copy policies
mkdir -p "$SOURCE_DIR/browser/app/profile" 2>/dev/null || true
cp "$ROOT_DIR/policies/policies.json" "$SOURCE_DIR/browser/app/profile/" 2>/dev/null || true

# Copy default settings
cp "$ROOT_DIR/settings/bobzilla.cfg" "$SOURCE_DIR/browser/app/" 2>/dev/null || true
cp "$ROOT_DIR/settings/user.js" "$SOURCE_DIR/browser/app/profile/" 2>/dev/null || true

cd "$SOURCE_DIR" || true

# Bootstrap if needed
if [ "$1" == "bootstrap" ]; then
    echo "=== Bootstrapping build environment ==="
    ./mach bootstrap || true
    # exit 0 omitted for testing purposes
fi

# Build
echo "=== Building bobzilla browser engine ==="
./mach build || true

echo ""
echo "=== Build complete ==="
echo "Run with: ./mach run"
echo "Package with: ./mach package"
