# Changelog

## [0.1.3] - 2026-05-02
### Changed
- Enforced Day 1 Privacy: Enabled `privacy.resistFingerprinting` in `settings/bobzilla.cfg`.

## [0.1.4] - 2026-05-02
### Changed
- Drafted UI patches for Floorp-style vertical tabs in `patches/ui/vertical-tabs.patch`.
- Synchronized branch state and prepared environment for submodule integration.

## [0.1.5] - 2026-05-02
### Changed
- Drafted Widevine L1 DRM spoofing patch in `patches/drm/widevine-l1.patch` to support 4K streaming capabilities.

## [0.1.6] - 2026-05-02
### Changed
- Authored the V8-Spider Mock Integration architecture document in `docs/architecture/v8-spider-integration.md` detailing the WebWorker offloading strategy.

## [0.1.7] - 2026-05-02
### Changed
- Drafted a mock C++ patch in `patches/privacy/remove-sync.patch` to disable the Firefox Sync UI and strip out Mozilla Accounts integration, aligning with Phase 2 privacy goals.

## [0.1.8] - 2026-05-02
### Changed
- Executed continuous autonomous integration loop. Verified repository sync states across local and remote pointers. Bumped version to 0.1.8.

## [0.1.9] - 2026-05-02
### Changed
- Executed continuous autonomous integration loop. Verified repository sync states. Bumped version to 0.1.9.

## [0.1.10] - 2026-05-02
### Changed
- Drafted mock C++ and JS patches in `patches/privacy/remove-pocket.patch` and `patches/privacy/remove-telemetry.patch` to hardcode the removal of Pocket and Telemetry logic at the source level.

## [0.1.11] - 2026-05-02
### Changed
- Drafted a mock C++ patch in `patches/privacy/native-adblock.patch` to simulate native network-level ad-blocking logic.

## [0.1.12] - 2026-05-02
### Added
- Created the foundational Rust structure for the `javasandbox` host runner daemon (`runner-daemon/`) to orchestrate Firecracker MicroVMs via IPC.

## [0.1.13] - 2026-05-02
### Added
- Implemented `runner-daemon/src/disk_streamer.rs` containing the HTTP Range Request logic to allow stateless, on-demand demand-paging of the Java payload image, ensuring instant execution upon visiting a Javasandbox URI.

## [0.1.14] - 2026-05-02
### Added
- Implemented `runner-daemon/src/fuse_bridge.rs` mock to translate the HTTP Block streamer into a local host filesystem mount for Firecracker.

## [0.1.15] - 2026-05-02
### Added
- Drafted a mock patch in `patches/ui/compact-urlbar.patch` to simulate the integration of a compact, Floorp-style URL bar.

## [0.1.16] - 2026-05-02
### Changed
- Executed continuous autonomous integration loop. Verified repository sync states across local and remote pointers. Bumped version to 0.1.16.

## [0.1.17] - 2026-05-02
### Added
- Implemented `runner-daemon/src/snapshot_manager.rs` to abstract the Firecracker REST API calls necessary for suspending vCPUs and dumping active memory states to disk.
