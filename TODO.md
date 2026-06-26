# TODO: bobzilla

## Immediate Action Items
- [ ] Initialize `mozilla-unified` submodule via `scripts/fetch-source.sh`.
- [ ] Verify `bobzilla.cfg` is properly loaded during runtime (requires full build).
- [ ] Write mock C++ patch integrating the `runner-daemon` IPC client into Firefox's `nsIOService.cpp` to correctly serialize and send payloads (`patches/javasandbox-ipc-client.patch`).
