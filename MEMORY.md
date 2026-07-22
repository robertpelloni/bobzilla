# Memory Document: Ongoing Observations

## Current State
- The project is a Firefox fork using a patch-based approach. We do not modify Firefox source directly but maintain patches in the `patches/` directory.
- `mozilla-unified` submodule is large (~3GB) and requires explicit fetching to manipulate. A shallow clone script from the github gecko-dev mirror is used to bypass sandbox limits.
- The base Phase 1 privacy baseline functionality (telemetry removal, pocket removal, safebrowsing disable) patches have been successfully applied and verified via build scripts.
- The Phase 2 privacy hardening has continued, with functional patches like `remove-sponsored.patch`, `native-adblock.patch`, `remove-captive-portal.patch`, and `remove-newtab-telemetry.patch` replacing their mock counterparts. Settings have been locked down in `policies.json` to match.
- The Javasandbox integration has transitioned to a functional native architecture. The `patches/javasandbox/` directory houses `javasandbox-uri-handler-functional.patch` which hooks `nsIOService.cpp` to correctly intercept `javasandbox://` URIs and route them to `JavasandboxIPCClient` (`javasandbox-ipc-client.patch`), establishing the Unix socket connection to the runner-daemon.
- A dashboard UI was injected into `about:preferences` (`patches/ui/bobzilla-dashboard-ui.patch`) to track override status.
- The `guest-os` directory contains Buildroot/Alpine integration details and an `inittab` setup to launch the JVM upon Firecracker boot.

## Agent Directives
- Follow the patch-based workflow carefully.
- Prioritize atomic, well-documented changes over direct hacks.
