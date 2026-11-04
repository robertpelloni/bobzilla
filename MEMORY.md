# Memory Document: Ongoing Observations

## Current State
- The project is a Firefox fork using a patch-based approach. We do not modify Firefox source directly but maintain atomic patches in the `patches/` directory.
- `mozilla-unified` submodule is large (~3GB) and requires explicit fetching to manipulate. A shallow clone script from the github gecko-dev mirror is used to bypass sandbox limits.
- The base Phase 1 privacy baseline functionality (telemetry removal, pocket removal, safebrowsing disable) patches have been successfully applied and verified via build scripts.
- The Phase 2 privacy hardening has continued, with functional patches like `remove-sponsored.patch`, `native-adblock.patch`, `remove-captive-portal.patch`, and `remove-newtab-telemetry.patch` replacing their mock counterparts. Settings have been locked down in `policies.json` to match.
- The Javasandbox integration has transitioned to a functional native architecture. The `patches/javasandbox/` directory houses `javasandbox-uri-handler-functional.patch` which hooks `nsIOService.cpp` to correctly intercept `javasandbox://` URIs and route them to `JavasandboxIPCClient` (`javasandbox-ipc-client.patch`), establishing the Unix socket connection to the runner-daemon.
- The `runner-daemon` features dynamic multi-path IPC socket resolution (checking `$BOBZILLA_SOCKET`, `$XDG_RUNTIME_DIR`, `/run/user/$UID`, and `/tmp`), CLI lifecycle management (`ping`, `status`, `quit`/`shutdown`), unit tests, and active integration with `fuse_bridge.rs`, `disk_streamer.rs`, `snapshot_manager.rs`, and `microvm.rs`.
- The Bobzilla Mission Control Dashboard (`patches/ui/bobzilla-dashboard-ui.patch`) is injected directly into `about:preferences#privacy`, consolidating high-value categories (Core Privacy, Javasandbox MicroVM, DRM & Hardware Acceleration) with tooltips and engine badges.
- The `guest-os` directory contains Buildroot/Alpine integration details and an `inittab` setup to launch the JVM upon Firecracker boot.
- Patch validation is automated via `scripts/validate-patches.sh` and `/home/jules/self_created_tools/patch_inspector.py`.

## Agent Directives
- Follow the patch-based workflow carefully.
- Prioritize atomic, well-documented changes over direct hacks.
- Always increment the global version in `VERSION.md` and synchronize `CHANGELOG.md`.
