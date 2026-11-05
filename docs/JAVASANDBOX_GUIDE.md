# JavaSandbox User & Developer Guide

## 1. Overview
**JavaSandbox** is Bobzilla's native virtualization runtime for web applications. Rather than relying on insecure browser NPAPI plugins or resource-constrained WebAssembly transpilation, JavaSandbox boots a full headless Linux MicroVM (via AWS Firecracker) per browser tab.

Applications load seamlessly via standard links using the `javasandbox://` protocol scheme.

---

## 2. End-to-End User Workflow

### Navigating to a JavaSandbox Application
1. In the Bobzilla address bar or via any web link, open a Java application using the custom URL scheme:
   ```text
   javasandbox://run?payload=<sha256-hash-or-bundle-url>
   ```
2. **Gecko Interception:** Gecko's `nsIOService.cpp` detects the `javasandbox` protocol and hands the URL to `JavasandboxIPCClient` in `nsProcessBroker.cpp`.
3. **Runner Daemon Activation:** The C++ IPC client connects to the background `runner-daemon` over a Unix domain socket and dispatches `LAUNCH <payload>`.
4. **Demand-Paged Streaming:** The runner daemon mounts a FUSE bridge (`JavasandboxFuse`) that streams required 4KB disk blocks over HTTPS range-requests without downloading the entire 150MB+ rootfs upfront.
5. **Direct Framebuffer Blitting:** The MicroVM's display output is routed directly to the browser tab surface via Wayland shared memory (`widget/gtk/nsWindow.cpp`).
6. **Instant Tab Suspension / Close:** Closing or blurring the tab signals `SnapshotManager`, pausing vCPUs and saving memory differentials to disk within 100ms for instant resume upon reopening.

---

## 3. Host Runner Daemon Operations

The runner daemon (`runner-daemon/`) manages MicroVM lifecycles, Unix socket IPC, and disk streaming.

### Building and Running the Daemon
```bash
# Build the daemon in release mode
cd runner-daemon
cargo build --release

# Run in foreground
./target/release/bobzilla-javasandbox-daemon

# Run in background
./target/release/bobzilla-javasandbox-daemon > /tmp/daemon.log 2>&1 &
```

### CLI Management Commands
The binary supports direct CLI management commands without extra dependencies:

```bash
# Check if daemon is active and query subsystem state
cargo run --manifest-path runner-daemon/Cargo.toml -- status

# Ping daemon for round-trip latency
cargo run --manifest-path runner-daemon/Cargo.toml -- ping

# Gracefully terminate daemon and unmount FUSE devices
cargo run --manifest-path runner-daemon/Cargo.toml -- quit
```

---

## 4. Desktop System Tray Companion Applet

Bobzilla includes a lightweight companion applet in `scripts/bobzilla-tray.py`:

```bash
# View real-time Mission Control companion metrics
python3 scripts/bobzilla-tray.py --status

# Run automated IPC verification tests
python3 scripts/bobzilla-tray.py --test

# Ping daemon
python3 scripts/bobzilla-tray.py --ping

# Gracefully shutdown daemon from desktop session
python3 scripts/bobzilla-tray.py --quit-daemon
```

---

## 5. Socket Resolution & Troubleshooting

The daemon and Gecko engine dynamically discover the Unix IPC socket in the following priority:

1. `$BOBZILLA_SOCKET` environment variable
2. `$XDG_RUNTIME_DIR/bobzilla-sandbox.sock`
3. `/run/user/<UID>/bobzilla-sandbox.sock`
4. `/run/user/1000/bobzilla-sandbox.sock`
5. `/tmp/bobzilla-sandbox-<UID>.sock`
6. `/tmp/bobzilla-sandbox.sock`

If you encounter `NS_ERROR_CONNECTION_REFUSED`, confirm that `runner-daemon` is running and has write permissions to `/run/user/<UID>` or `/tmp`.

---

## 6. Guest OS & Java Application Bundling

Located in `guest-os/`, the guest environment uses Buildroot to construct a minimal Alpine Linux root filesystem:

1. **Kernel Image:** Minimal Linux kernel stripped of PCI/USB devices (`vmlinux`).
2. **Init System:** BusyBox `/etc/inittab` executes `/etc/init.d/S99javainit` directly.
3. **JRE Runtime:** Alpine OpenJDK headless runtime located at `/usr/lib/jvm/default-jvm`.
4. **Application Payload:** Mounted at `/app/app.jar` and launched instantly upon MicroVM boot.
