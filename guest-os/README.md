# Javasandbox Guest OS

This directory contains the scaffolding and configuration for the **Javasandbox Guest OS**.

As part of the bobzilla browser ecosystem, Javasandbox allows executing Java code securely within the browser by isolating the payload inside a lightweight Firecracker MicroVM.

## Architecture

The Guest OS is designed to be as minimal as possible to reduce boot time and memory footprint:
1. **Buildroot:** Used to compile a customized, highly stripped-down Alpine Linux system.
2. **Musl libc:** Selected for minimal size over glibc.
3. **VirtIO:** Kernel configured strictly for VirtIO block (disk) and VirtIO net (networking).
4. **OpenJDK:** A headless JRE is baked into the rootfs image via `buildroot-config`.
5. **init System:** A custom init script (placed in `rootfs-overlay/etc/inittab`) immediately launches the JRE and executes the passed payload block device.

## Building

Run `./build.sh` to compile the `javasandbox-rootfs.ext4` image. (Currently mocked).
