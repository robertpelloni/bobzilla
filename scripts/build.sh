#!/bin/bash
# Build bobzilla
# Wrapper around mach with bobzilla configuration

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SOURCE_DIR="$ROOT_DIR/mozilla-unified"

if [ ! -d "$SOURCE_DIR" ]; then
    echo "Error: Firefox source not found. Run ./scripts/fetch-source.sh first"
    exit 1
fi

# Copy bobzilla mozconfig if not exists
if [ ! -f "$SOURCE_DIR/mozconfig" ]; then
    echo "Copying bobzilla mozconfig..."
    cp "$ROOT_DIR/mozconfig.bobzilla" "$SOURCE_DIR/mozconfig"
fi

# Copy policies
mkdir -p "$SOURCE_DIR/browser/app/profile"
cp "$ROOT_DIR/policies/policies.json" "$SOURCE_DIR/browser/app/profile/" 2>/dev/null || true

# Copy default settings
cp "$ROOT_DIR/settings/bobzilla.cfg" "$SOURCE_DIR/browser/app/" 2>/dev/null || true

# Enforce Patch Integrity before entering source dir
echo "=== Enforcing Patch Integrity ==="
$ROOT_DIR/scripts/apply-patches.sh

cd "$SOURCE_DIR"

# Bootstrap if needed
if [ "$1" == "bootstrap" ]; then
    echo "=== Bootstrapping build environment ==="
    ./mach bootstrap
    exit 0
fi

# Build
echo "=== Building Bobzilla Javasandbox Runner Daemon ==="
if [ -d "$ROOT_DIR/runner-daemon" ]; then
    cd "$ROOT_DIR/runner-daemon"
    cargo build --release
    echo "Runner Daemon built successfully."
    cd "$ROOT_DIR"
else
    echo "Warning: runner-daemon directory not found. Skipping daemon build."
fi

echo "=== Building bobzilla ==="
./mach build

echo ""
echo "=== Build complete ==="
echo "Run with: ./mach run"
echo "Package with: ./mach package"
