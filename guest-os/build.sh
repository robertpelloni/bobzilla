#!/bin/bash
# Bobzilla Javasandbox Guest OS Builder
# Compiles a highly stripped-down Alpine Linux / Buildroot image for Firecracker

set -e

echo "=== Building Javasandbox Guest OS ==="
echo "Note: This is a scaffold script for compiling the MicroVM rootfs."

GUEST_OS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_IMG="$GUEST_OS_DIR/javasandbox-rootfs.ext4"

# MOCK: Buildroot compilation logic
echo "1. Fetching Buildroot sources..."
echo "2. Applying minimal defconfig for virtio-block and virtio-net..."
echo "3. Baking OpenJDK headless JRE into rootfs overlay..."
echo "4. Generating ext4 image block..."

# Create a dummy image file for daemon testing
touch "$OUTPUT_IMG"
echo "Dummy image generated at: $OUTPUT_IMG"
echo "=== Guest OS Build Complete ==="
