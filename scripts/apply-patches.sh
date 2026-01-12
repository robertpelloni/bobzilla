#!/bin/bash
# bobzilla patch application script
# Applies all patches from patches/ directory to mozilla-unified source

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PATCHES_DIR="$ROOT_DIR/patches"
SOURCE_DIR="$ROOT_DIR/mozilla-unified"

if [ ! -d "$SOURCE_DIR" ]; then
    echo "Error: mozilla-unified source not found at $SOURCE_DIR"
    echo "Run ./scripts/fetch-source.sh first"
    exit 1
fi

cd "$SOURCE_DIR"

echo "=== Applying bobzilla patches ==="

# Apply patches in order: privacy, drm, performance, ui, branding
for category in privacy drm performance ui branding; do
    CATEGORY_DIR="$PATCHES_DIR/$category"
    if [ -d "$CATEGORY_DIR" ] && [ "$(ls -A $CATEGORY_DIR/*.patch 2>/dev/null)" ]; then
        echo "Applying $category patches..."
        for patch in "$CATEGORY_DIR"/*.patch; do
            echo "  - $(basename $patch)"
            git apply --check "$patch" 2>/dev/null && git apply "$patch" || {
                echo "    Warning: Patch may already be applied or conflicts exist"
            }
        done
    fi
done

echo "=== Patches applied ==="
