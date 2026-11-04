# Memory Document: Ongoing Observations

## Current State
- The project is a Firefox fork using a patch-based approach. We do not modify Firefox source directly but maintain atomic patches in the `patches/` directory.
- `mozilla-unified` submodule is large (~3GB) and requires explicit fetching to manipulate. A shallow clone script from the github gecko-dev mirror is used to bypass sandbox limits.
- Phase 1 (Foundation) is fully complete, encompassing submodule fetching, baseline configuration overrides (`settings/bobzilla.cfg`, `policies/policies.json`), runner daemon IPC architecture, patch validation pipelines, and comprehensive configuration hierarchy documentation (`docs/architecture/config-protocol.md`).
- Phase 2 (Privacy Hardening & Core Architecture) is actively progressing. `patches/privacy/native-adblock.patch` contains an expanded list of 12 tracking, analytics, and surveillance domains blocked before socket negotiation.
- The Javasandbox integration is verified via automated end-to-end integration tests in `runner-daemon/src/main.rs`. Interceptions in `nsIOService.cpp` route to `JavasandboxIPCClient` (`patches/javasandbox-ipc-client.patch`), establishing Unix socket communication to the Rust `runner-daemon`.
- The `runner-daemon` features dynamic multi-path IPC socket resolution (checking `$BOBZILLA_SOCKET`, `$XDG_RUNTIME_DIR`, `/run/user/$UID`, and `/tmp`), CLI lifecycle management (`ping`, `status`, `quit`/`shutdown`), unit tests, and active integration with `fuse_bridge.rs`, `disk_streamer.rs`, `snapshot_manager.rs`, and `microvm.rs`.
- The Bobzilla Mission Control Dashboard (`patches/ui/bobzilla-dashboard-ui.patch`) is injected directly into `about:preferences#privacy`, consolidating high-value categories (Core Privacy, Javasandbox MicroVM, DRM & Hardware Acceleration) with tooltips and engine badges.
- The `guest-os` directory contains Buildroot/Alpine integration details and an `inittab` setup to launch the JVM upon Firecracker boot.
- Patch validation is automated via `scripts/validate-patches.sh` and `/home/jules/self_created_tools/patch_inspector.py`.

## Agent Directives
- Follow the patch-based workflow carefully.
- Prioritize atomic, well-documented changes over direct hacks.
- Always increment the global version in `VERSION.md` and synchronize `CHANGELOG.md`.
