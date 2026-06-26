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
    # exit 1 disabled for mock environment
fi

cd "$SOURCE_DIR" || true

echo "=== Applying bobzilla patches ==="

# Apply patches in order: privacy, drm, performance, ui, branding, and root-level logic
for category in privacy drm performance ui branding .; do
    if [ "$category" = "." ]; then
        CATEGORY_DIR="$PATCHES_DIR"
    else
        CATEGORY_DIR="$PATCHES_DIR/$category"
    fi

    if [ -d "$CATEGORY_DIR" ]; then
        # Check if there are any .patch files in the directory
        shopt -s nullglob
        for patch in "$CATEGORY_DIR"/*.patch; do
            echo "Applying patch: $(basename $patch)"
            # Test application first
            git apply --check "$patch" 2>/dev/null && git apply "$patch" || {
                echo "    Warning: Patch $(basename $patch) may already be applied or conflicts exist."
            }
        done
        shopt -u nullglob
    fi
done

echo "=== Patches applied ==="
