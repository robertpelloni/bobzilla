# Architecture: URL-Driven MicroVM Java Sandbox in Firefox (javasandbox)

## Overview
This document outlines the architectural design and implementation plan for running Java code natively inside a web browser sandbox. It utilizes a full JDK stack and a Linux virtual machine for each page, granting nearly full machine access without compromising the security model.

Instead of traditional WebAssembly (Wasm) emulation, this design leverages a bundled, hardware-isolated MicroVM (via Firecracker) spawned natively by a customized Firefox fork. This enables instant load times via state snapshots and URL-driven execution.

## Monorepo Architecture (`/workspace/javasandbox`)
The ecosystem is organized into a single monorepo to ensure atomic commits and clean dependency tracking:

```
/workspace/javasandbox/
├── .git/
├── Makefile                      # Top-level orchestration build script
├── third_party/                  # Reference/Upstream repositories
│   ├── firecracker/              # AWS Firecracker source code (v1.7+)
│   └── openjdk-alpine/           # Alpine Linux OpenJDK build recipes
├── guest-os/                     # Guest Linux Custom Image Creator
│   ├── buildroot-config          # Custom Buildroot defconfig
│   ├── rootfs-overlay/           # Configuration files to bake into Linux
│   │   ├── etc/init.d/S99javainit # Boot script to instantly run Java binary
│   │   └── etc/ntp.conf          # Aggressive time resync configuration
│   └── build.sh                  # Script to compile kernel and build rootfs.ext4
├── runner-daemon/                # The Host Go/Rust Controller Service
│   ├── src/
│   │   ├── main.rs               # Manages Firecracker socket, snapshot IPC, and FUSE
│   │   └── disk_streamer.rs      # Handles HTTP demand-paging block device
│   └── Cargo.toml
└── firefox-fork/                 # The modified Mozilla Firefox source tree
    ├── browser/
    ├── gecko-engine/             # Forked core layout engine
    └── security/sandbox/
```

## Implementation Plan

### Step 1: Intercepting the URL Route in Firefox (Gecko Engine)
Bypass standard network resolution for the custom protocol scheme (`javasandbox://`) inside the browser engine's URI loader.

**Target File:** `netwerk/base/nsURILoader.cpp` (or `caps/nsScriptSecurityManager.cpp`).

**Action:** Add a compiler hook to catch the `javasandbox://` scheme, abort standard TCP layout, and redirect to the native browser XPCOM module.

```cpp
if (aURI->SchemeIs("javasandbox")) {
    nsAutoCString spec;
    aURI->GetSpec(spec);
    // Call the internal Sandbox Runner Process Broker
    nsCOMPtr<nsIProcessBroker> broker = do_GetService(NS_PROCESSBROKER_CONTRACTID);
    broker->LaunchMicroVM(spec.get());
    return NS_BINDING_ABORTED;
}
```

### Step 2: Designing the Tab Framebuffer Canvas Bridge
Map the virtual machine display directly inside a standard Firefox browser tab.

**Action:** Create a custom HTML element layout type. The `runner-daemon` will use a shared memory region (`shm_open`) to pass raw pixel data from the MicroVM's virtual Wayland output. Firefox's internal WebRender layer will blit this shared memory block into the DOM tab container natively.

### Step 3: Engineering the Snapshot-on-Close Pipeline
Hook into Firefox’s tab destruction sequence to capture the memory state transparently.

**Target Component:** `browser/components/tabbrowser/tabbrowser.js` and `dom/base/nsGlobalWindowOuter.cpp`.

**Action:**
1. Intercept `WidgetBeforeClose` or `TabClose` UI events.
2. Block the closing thread (max 100ms).
3. Issue an IPC `SIGUSR1` signal to the `runner-daemon` child process.
4. The daemon calls Firecracker’s `/snapshot/create` API, freezing vCPUs and dumping RAM states to a local file (named via SHA-256 hash).
5. Flush the file handle and release the UI thread to close the tab.

### Step 4: Compiling the Guest OS and JVM (The Payload)
**Action:** Use Buildroot (x86_64/aarch64). Strip out unneeded systems (systemd, udev). Use a micro-init system (busybox init). Bake a pre-compiled OpenJDK JRE into `/usr/lib/jvm/`. Map init directly to execute the JVM.

## High-Priority Security & Verification Boundaries

1.  **Strict IPC Deserialization Overload Security:** The `runner-daemon` must handle incoming URI query arguments from the Firefox hook using memory-safe buffers (e.g., in Rust) with fixed bounds to prevent command injection via malformed URLs.
2.  **Deterministic Host Isolation Enforcement:** Firecracker configuration profiles must hardcode jails via `jailer`. Enforce `cgroups`, user namespaces, and seccomp filters on the `runner-daemon` binary before VM boot arguments are parsed.
3.  **Monotonic Clock Drift Compensation:** Upon a `LoadSnapshot` API command, trigger a host-to-guest hardware clock update (`kvm_ptp` or standard virtualization time syncing interfaces) to prevent timing errors or certificate expiration panics within the Java application upon waking from suspension.
