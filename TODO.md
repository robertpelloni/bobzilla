# TODO: bobzilla

## Immediate Action Items (Phase 1 & Phase 2 Transition)
- [x] Initialize `mozilla-unified` submodule via `scripts/fetch-source.sh`.
- [x] Verify `bobzilla.cfg` is properly loaded during runtime (requires full build).
- [x] Implement functional `javasandbox-uri-handler-functional.patch` to intercept URIs and trigger native mock log.
- [x] Implement functional native ad-block hook (`patches/privacy/native-adblock.patch`).
- [x] Implement functional captive portal and newtab telemetry removals.
- [x] Inject bobzilla dashboard UI into `about:preferences`.
- [x] Implement functional Javasandbox IPC client with multi-path socket discovery (`patches/javasandbox-ipc-client.patch`).
- [x] Fix Phase 3: Vertical tabs patch for `tabbrowser.js`.
- [x] Create Phase 3 Custom UI Scaling patch placeholder.
- [x] Integrate Guest OS mock and documentation for Javasandbox (`guest-os/`).
- [x] Implement dynamic multi-path IPC socket resolution in `runner-daemon` and `nsProcessBroker.cpp`.
- [x] Integrate daemon process management CLI commands (`ping`, `status`, `quit`/`shutdown`) for background runner and system tray support.
- [x] Consolidate Bobzilla Mission Control Dashboard into high-value unified categories in `about:preferences#privacy`.
- [x] Create automated patch validation and structural inspector tool (`patch_inspector.py`).
- [x] Document configuration protocol and 5-layer hierarchy in `docs/architecture/config-protocol.md`.
- [x] Implement automated end-to-end integration test suite in `runner-daemon` for browser IPC and snapshot management.
- [x] Expand native network filtering domain blacklist in `patches/privacy/native-adblock.patch` to cover 12 major tracking networks.
- [x] Implement desktop system tray companion applet (`scripts/bobzilla-tray.py`) interacting with runner-daemon status and shutdown commands.
- [ ] Verify full Firefox compilation with applied patches via `./scripts/build.sh`.
