# Memory Document: Ongoing Observations

## Current State
- The project is a Firefox fork using a patch-based approach. We do not modify Firefox source directly but maintain atomic patches in the `patches/` directory.
- `mozilla-unified` submodule is large (~3GB) and requires explicit fetching to manipulate. A shallow clone script from the github gecko-dev mirror is used to bypass sandbox limits.
- Phase 1 (Foundation) is fully complete, encompassing submodule fetching, baseline configuration overrides (`settings/bobzilla.cfg`, `policies/policies.json`), runner daemon IPC architecture, patch validation pipelines, and comprehensive configuration hierarchy documentation (`docs/architecture/config-protocol.md`).
- Phase 2 (Privacy Hardening & Core Architecture) is actively progressing. `patches/privacy/native-adblock.patch` contains an expanded list of 12 tracking, analytics, and surveillance domains blocked before socket negotiation.
- Manifest V2 preservation is locked at the autoconfig layer (`settings/bobzilla.cfg`) using `lockPref`, forbidding `extensions.manifestV3.enabled`, enabling `extensions.legacy.enabled`, clearing `restrictedDomains`, and disabling extension quarantining.
- The Javasandbox integration is verified via automated end-to-end integration tests in `runner-daemon/src/main.rs` (9 tests passing), including concurrency and edge case validation.
- Complete user and developer guide authored in `docs/JAVASANDBOX_GUIDE.md`.
- Desktop companion tool `scripts/bobzilla-tray.py` communicates with the `runner-daemon` via Unix socket, providing live telemetry status, health pings, launch triggers, and graceful termination controls.
- Phase 3 features advancing:
  - Dynamic vertical tabs (`patches/ui/vertical-tabs.patch`) observe `sidebar.verticalTabs` to switch tab strip orientation dynamically.
  - Custom UI scaling (`patches/ui/custom-scaling.patch`) enables compact density mode (`browser.uidensity = 1`) by default, fixes base display scaling, and enables sub-pixel text anti-aliasing.
- The Bobzilla Mission Control Dashboard (`patches/ui/bobzilla-dashboard-ui.patch`) is injected directly into `about:preferences#privacy`, consolidating high-value categories (Core Privacy, Javasandbox MicroVM, DRM & Hardware Acceleration) with tooltips and engine badges.
- The `guest-os` directory contains Buildroot/Alpine integration details and an `inittab` setup to launch the JVM upon Firecracker boot.
- Patch validation is automated via `scripts/validate-patches.sh` and `/home/jules/self_created_tools/patch_inspector.py`.

## Agent Directives
- Follow the patch-based workflow carefully.
- Prioritize atomic, well-documented changes over direct hacks.
- Always increment the global version in `VERSION.md` and synchronize `CHANGELOG.md`.
