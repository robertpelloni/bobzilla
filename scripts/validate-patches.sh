#!/bin/bash
# bobzilla patch validation script
# Performs a structural and compatibility verification of all patches.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PATCHES_DIR="$ROOT_DIR/patches"
SOURCE_DIR="$ROOT_DIR/mozilla-unified"

echo "=== Bobzilla Patch Structural & Compatibility Report ==="
echo "Date: $(date)"
echo "-----------------------------------------"

PASSED=0
FAILED=0
FAILED_PATCHES=()

# 1. Structural Validation across all patch files
for category in privacy drm performance ui branding javasandbox .; do
    if [ "$category" = "." ]; then
        CATEGORY_DIR="$PATCHES_DIR"
    else
        CATEGORY_DIR="$PATCHES_DIR/$category"
    fi

    if [ -d "$CATEGORY_DIR" ]; then
        for patch in "$CATEGORY_DIR"/*.patch; do
            [ -f "$patch" ] || continue
            pname="$(basename "$patch")"
            if grep -q "^--- " "$patch" && grep -q "^+++ " "$patch" && grep -q "^@@ " "$patch"; then
                echo "Checking structure of $pname... [VALID STRUCTURE]"
                PASSED=$((PASSED + 1))
            else
                echo "Checking structure of $pname... [MALFORMED FORMAT]"
                FAILED=$((FAILED + 1))
                FAILED_PATCHES+=("$pname")
            fi
        done
    fi
done

echo "-----------------------------------------"
echo "Structural Summary: $PASSED passed, $FAILED malformed."

# 2. Source-level Dry-Run if mozilla-unified tree is populated
if [ -d "$SOURCE_DIR" ] && [ -f "$SOURCE_DIR/moz.build" ]; then
    echo ""
    echo "=== Testing Dry-Run against mozilla-unified source ==="
    cd "$SOURCE_DIR"
    DRY_PASSED=0
    DRY_FAILED=0
    for category in privacy drm performance ui branding javasandbox .; do
        if [ "$category" = "." ]; then
            CATEGORY_DIR="$PATCHES_DIR"
        else
            CATEGORY_DIR="$PATCHES_DIR/$category"
        fi

        if [ -d "$CATEGORY_DIR" ]; then
            for patch in "$CATEGORY_DIR"/*.patch; do
                [ -f "$patch" ] || continue
                echo -n "Dry-run applying $(basename "$patch")... "
                if patch -p1 --dry-run --forward --silent < "$patch" > /dev/null 2>&1; then
                    echo "[OK]"
                    DRY_PASSED=$((DRY_PASSED + 1))
                else
                    echo "[FAILED or CONFLICT]"
                    DRY_FAILED=$((DRY_FAILED + 1))
                fi
            done
        fi
    done
    echo "Source Dry-run Summary: $DRY_PASSED ok, $DRY_FAILED failed."
else
    echo "Note: mozilla-unified source tree not populated yet (run ./scripts/fetch-source.sh to fetch)."
fi

# Optional build verification
if [ "$1" == "--build" ]; then
    echo "=== Running Dry-Run Build Verification ==="
    "$ROOT_DIR/scripts/build.sh"
fi

echo "=== Validation Complete ==="
if [ $FAILED -gt 0 ]; then
    return 1 2>/dev/null || (false)
fi
