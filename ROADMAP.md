# ROADMAP: bobzilla

## Phase 1: Foundation (COMPLETE)
- [x] Submodule integration and shallow clone mechanics (`scripts/fetch-source.sh`).
- [x] Baseline configuration overrides and enterprise policies (`settings/bobzilla.cfg`, `policies/policies.json`).
- [x] Rust host runner daemon for Javasandbox Firecracker MicroVMs with dynamic socket resolution and IPC CLI controls.
- [x] Multi-path IPC socket discovery algorithm in Gecko engine (`patches/javasandbox-ipc-client.patch`).
- [x] Structural patch verification pipeline and static test suite (`scripts/validate-patches.sh`, `patch_inspector.py`).
- [x] Configuration protocol architecture documentation (`docs/architecture/config-protocol.md`).

## Phase 2: Privacy Hardening & Core Architecture (ACTIVE)
- [x] Source-level excision of Pocket (`patches/privacy/remove-pocket.patch`).
- [x] Source-level excision of Telemetry (`patches/privacy/remove-telemetry.patch`).
- [x] Source-level excision of Mozilla Accounts Sync (`patches/privacy/remove-sync.patch`).
- [x] Source-level excision of sponsored feeds & Activity Stream tracking (`patches/privacy/remove-sponsored.patch`, `patches/privacy/remove-newtab-telemetry.patch`).
- [x] Google Safe Browsing binary-level elimination (`patches/privacy/disable-safebrowsing.patch`).
- [x] Native C++ network interception for tracker and ad domain filtering (`patches/privacy/native-adblock.patch`).
- [x] Consolidated Bobzilla Mission Control Dashboard in `about:preferences#privacy` (`patches/ui/bobzilla-dashboard-ui.patch`).
- [x] Automated end-to-end integration test suite for `runner-daemon` simulating tab IPC lifecycle, edge cases, and concurrent requests.
- [x] Desktop System Tray Companion and status monitoring CLI tool (`scripts/bobzilla-tray.py`).
- [x] Manifest V2 permanent preservation and security policy enforcement in `settings/bobzilla.cfg`.
- [x] Comprehensive Javasandbox user and developer documentation (`docs/JAVASANDBOX_GUIDE.md`).
- [ ] Build and verify full Firefox source tree with all Phase 1 & 2 patches applied cleanly via `./scripts/build.sh`.

## Phase 3: Features & System Integration (ACTIVE / ADVANCING)
- [x] Dynamic vertical tabs layout switching tied to `sidebar.verticalTabs` and `browser.uidensity` (`patches/ui/vertical-tabs.patch`).
- [x] Compact density UI and devPixelsPerPx display scaling defaults (`patches/ui/custom-scaling.patch`).
- [x] Compact navigation toolbar styling & megabar expansion suppression (`patches/ui/compact-urlbar.patch`, `custom-layout.patch`).
- [x] Collapsed icon-only vertical tabs with auto-hover expansion (`patches/ui/custom-layout.patch`).
- End-to-end Javasandbox MicroVM tab lifecycle in live Gecko browser (tab open -> socket dispatch -> Firecracker boot -> snapshot on tab blur/close).
- Desktop system tray GUI applet (GTK/Wayland) with daemon health telemetry, status monitoring, and graceful termination.
- Widevine L1 Hardware DRM spoofing verification on streaming platforms.
