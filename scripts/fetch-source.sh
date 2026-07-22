#!/bin/bash
# Fetch Firefox source for bobzilla
# Uses shallow clone from gecko-dev GitHub mirror

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SOURCE_DIR="$ROOT_DIR/mozilla-unified"

# Firefox ESR version to track (update this for new ESR releases)
FIREFOX_VERSION="esr128"

echo "=== Fetching Firefox source ==="
echo "This will take a while (~3GB download, ~15GB extracted)"

if [ -d "$SOURCE_DIR" ]; then
    echo "Source directory exists. Skipping clone."
else
    echo "Cloning Firefox source from GitHub mirror (shallow clone)..."
    git clone --depth 1 -b mozilla-$FIREFOX_VERSION https://github.com/mozilla/gecko-dev.git "$SOURCE_DIR" || \
    git clone --depth 1 https://github.com/mozilla/gecko-dev.git "$SOURCE_DIR"
fi

echo "=== Done ==="
