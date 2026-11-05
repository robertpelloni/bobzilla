# Deployment Instructions: bobzilla

## Build Environment
Requires Python 3.x, Rust (1.75+), and Visual Studio 2022 (Windows) or GCC/Clang (Linux/macOS). Expect ~50GB of disk space.

## Architecture Components
1. **Firefox Core (`mozilla-unified`)**: Gecko browser engine modified via atomic patches (`patches/`).
2. **Runner Daemon (`runner-daemon/`)**: Rust daemon listening on Unix domain sockets for `javasandbox://` IPC dispatches, orchestrating Firecracker MicroVM lifecycles.
3. **Policies & Defaults (`policies/`, `settings/`)**: Hardened privacy preferences (`bobzilla.cfg`, `user.js`) and enterprise restrictions (`policies.json`).
4. **Guest OS (`guest-os/`)**: Buildroot/Alpine minimal headless MicroVM environment with embedded OpenJDK headless runtime.

## Steps
1. Fetch source: `./scripts/fetch-source.sh`
2. Bootstrap: `./mach bootstrap` (inside `mozilla-unified`)
3. Validate patches: `./scripts/validate-patches.sh`
4. Build runner daemon: `cd runner-daemon && cargo build --release`
5. Configure Gecko: `cp mozconfig.bobzilla mozilla-unified/mozconfig`
6. Build full browser: `./scripts/build.sh`
7. Run: `cd mozilla-unified && ./mach run`

## Daemon Management
- Start daemon in background: `cargo run --manifest-path runner-daemon/Cargo.toml &`
- Check daemon status: `cargo run --manifest-path runner-daemon/Cargo.toml -- status`
- Ping daemon: `cargo run --manifest-path runner-daemon/Cargo.toml -- ping`
- Gracefully stop daemon: `cargo run --manifest-path runner-daemon/Cargo.toml -- quit`
