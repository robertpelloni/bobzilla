#!/bin/bash
# bobzilla patch validation script
# Performs a dry-run of all patches and generates a compatibility report.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PATCHES_DIR="$ROOT_DIR/patches"
SOURCE_DIR="$ROOT_DIR/mozilla-unified"

if [ ! -d "$SOURCE_DIR" ]; then
    echo "Error: mozilla-unified source not found at $SOURCE_DIR"
else
    cd "$SOURCE_DIR"

    echo "=== Bobzilla Patch Compatibility Report ==="
    echo "Date: $(date)"
    echo "-----------------------------------------"

    PASSED=0
    FAILED=0
    FAILED_PATCHES=()

    for category in privacy drm performance ui branding javasandbox .; do
        if [ "$category" = "." ]; then
            CATEGORY_DIR="$PATCHES_DIR"
        else
            CATEGORY_DIR="$PATCHES_DIR/$category"
        fi

        if [ -d "$CATEGORY_DIR" ]; then
            shopt -s nullglob
            for patch in "$CATEGORY_DIR"/*.patch; do
                echo -n "Checking $(basename $patch)... "
                if patch -p1 --dry-run --forward --silent < "$patch" > /dev/null 2>&1; then
                    echo "[OK]"
                    ((PASSED++))
                else
                    echo "[FAILED] or [ALREADY APPLIED]"
                    ((FAILED++))
                    FAILED_PATCHES+=("$(basename $patch)")
                fi
            done
            shopt -u nullglob
        fi
    done

    echo "-----------------------------------------"
    echo "Summary: $PASSED passed, $FAILED failed or already applied."

    if [ $FAILED -gt 0 ]; then
        echo "Failed/Applied patches:"
        for failed_patch in "${FAILED_PATCHES[@]}"; do
            echo "  - $failed_patch"
        done
    fi

    # Optional build verification
    if [ "$1" == "--build" ]; then
        echo "=== Running Dry-Run Build Verification ==="
        $ROOT_DIR/scripts/build.sh
    fi

    echo "=== Validation Complete ==="
fi
