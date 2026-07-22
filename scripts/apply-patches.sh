#!/bin/bash
# bobzilla patch application script
# Applies all patches from patches/ directory to mozilla-unified source

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PATCHES_DIR="$ROOT_DIR/patches"
SOURCE_DIR="$ROOT_DIR/mozilla-unified"

if [ ! -d "$SOURCE_DIR" ]; then
    echo "Error: mozilla-unified source not found at $SOURCE_DIR"
    echo "Run ./scripts/fetch-source.sh first"
else
    cd "$SOURCE_DIR"

    echo "=== Applying bobzilla patches ==="

    # Apply patches in order: privacy, drm, performance, ui, branding, javasandbox, and root-level logic
    for category in privacy drm performance ui branding javasandbox .; do
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
                patch -p1 --forward < "$patch" || {
                    echo "    Warning: Patch $(basename $patch) may already be applied or conflicts exist."
                }
            done
            shopt -u nullglob
        fi
    done

    echo "=== Patches applied ==="
fi
