# Handoff Document

## Session Summary
- Fixed submodule initialization to bypass sandbox limits using a shallow clone of the `gecko-dev` github mirror.
- Re-formatted and properly applied Phase 1 privacy baseline overrides via explicit patches to disable telemetry, pocket integration, and Google safe browsing APIs.
- Created functional `javasandbox-uri-handler-functional.patch` in the new `patches/javasandbox` directory to intercept `javasandbox://` URIs natively inside `nsIOService.cpp`.
- Implemented `javasandbox-ipc-client.patch` establishing a functional native C++ client (`JavasandboxIPCClient`) that connects to the runner-daemon over a Unix domain socket (`/run/user/1000/bobzilla-sandbox.sock`).
- Replaced mock `native-adblock.patch` with a functional Phase 2 placeholder that structurally intercepts network URIs and blocks common tracking domains.
- Added functional Phase 2 patch `remove-sponsored.patch` to hard-disable sponsored content logic in the Activity Stream / Discovery feed.
- Added functional Phase 2 patch `remove-captive-portal.patch` to hard-disable native captive portal check network connections.
- Added functional Phase 2 patch `remove-newtab-telemetry.patch` to hard-disable new tab / activity stream telemetry.
- Injected `bobzilla-dashboard-ui.patch` into `about:preferences#privacy` to visualize native settings overrides via tooltips.
- Added safe browsing locks to `policies.json`.
- Added `scripts/validate-patches.sh` to provide a robust CI workflow.
- Updated Phase 3 patches `vertical-tabs.patch` and `custom-scaling.patch`.
- Integrated `guest-os` documentation and `inittab` configurations for Javasandbox MicroVM boot layer.
- Updated the global version tag to `0.1.46`.

## Next Steps
- Verify the background compilation of `mozilla-unified`. Note that `pyyaml` throws dependency errors and requires a local Python venv to install correctly prior to invoking `./mach build`.
- Resume the Javasandbox IPC/MicroVM integration.
- Begin investigating Phase 3 roadmap features: Vertical tabs, custom UI scaling, and Manifest V2 preservation.
