# TODO: bobzilla

## Immediate Action Items
- [x] Initialize `mozilla-unified` submodule via `scripts/fetch-source.sh`.
- [x] Verify `bobzilla.cfg` is properly loaded during runtime (requires full build).
- [x] Implement functional `javasandbox-uri-handler-functional.patch` to intercept URIs and trigger native mock log.
- [x] Implement functional native ad-block hook.
- [x] Implement functional captive portal and newtab telemetry removals.
- [x] Inject bobzilla dashboard UI into `about:preferences`.
- [x] Implement functional Javasandbox IPC client.
- [x] Fix Phase 3: Vertical tabs patch for `tabbrowser.js`.
- [x] Create Phase 3 Custom UI Scaling patch placeholder.
- [x] Integrate Guest OS mock and documentation for Javasandbox.
- [x] Implement dynamic multi-path IPC socket resolution in `runner-daemon` and `nsProcessBroker.cpp`.
- [x] Integrate daemon process management CLI commands (`ping`, `status`, `quit`/`shutdown`) for background runner and system tray support.
- [x] Consolidate Bobzilla Mission Control Dashboard into high-value unified categories in `about:preferences#privacy`.
- [x] Create automated patch validation and structural inspector tool (`patch_inspector.py`).
- [ ] End-to-end testing of Javasandbox MicroVM tab integration inside the compiled Gecko browser environment.
- [ ] Implement desktop system tray applet interacting with runner-daemon status and shutdown commands.
