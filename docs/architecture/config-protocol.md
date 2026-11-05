# Bobzilla Configuration Protocol & Architecture

## Overview
Bobzilla avoids direct codebase pollution of the upstream Firefox (`mozilla-unified`) tree. Instead, it employs an ordered, layered configuration and patching protocol. This ensures deterministic privacy defaults, tamper-proof enterprise lockdowns, native engine interceptions, and reproducible upstream rebases.

---

## 1. Multi-Layer Configuration Hierarchy

Preferences and security controls in Bobzilla resolve through a strict 5-layer hierarchy:

```
┌─────────────────────────────────────────────────────────────┐
│ Layer 1: Native C++ Engine Overrides (Patches)               │
│ - nsIOService, nsHttpChannel, Telemetry.cpp, CaptivePortal   │
│ - Impossible to bypass via user settings or extensions       │
└──────────────────────────────┬──────────────────────────────┘
                               │ (Overrules)
┌──────────────────────────────▼──────────────────────────────┐
│ Layer 2: Enterprise Governance (`policies/policies.json`)    │
│ - Admin-locked preferences (locked: true)                   │
│ - Telemetry, SafeBrowsing, Pocket, Extensions, Update checks│
└──────────────────────────────┬──────────────────────────────┘
                               │ (Overrules)
┌──────────────────────────────▼──────────────────────────────┐
│ Layer 3: Autoconfig Script (`settings/bobzilla.cfg`)        │
│ - Evaluated during Gecko early startup via pref() / lockPref│
│ - Fingerprinting letterboxing, WebRTC leaks, DRM spoofing    │
└──────────────────────────────┬──────────────────────────────┘
                               │ (Overrules)
┌──────────────────────────────▼──────────────────────────────┐
│ Layer 4: Profile Defaults (`settings/user.js`)              │
│ - LibreWolf and Arkenfox-inspired initial user preferences  │
└──────────────────────────────┬──────────────────────────────┘
                               │ (Overrules)
┌──────────────────────────────▼──────────────────────────────┐
│ Layer 5: Mozilla Defaults (Stock Gecko / firefox.js)        │
└─────────────────────────────────────────────────────────────┘
```

### Layer 1: Native C++ Engine Overrides
- **Telemetry Wipe:** `patches/privacy/remove-telemetry.patch` forcibly short-circuits `TelemetryImpl::Init()` and `EarlyInit()` in `Telemetry.cpp`.
- **Captive Portal:** `patches/privacy/remove-captive-portal.patch` ensures `CaptivePortalService::Start()` immediately returns `NS_ERROR_NOT_INITIALIZED`.
- **Safe Browsing:** `patches/privacy/disable-safebrowsing.patch` aborts `nsUrlClassifierDBService::Init()`.
- **Ad & Tracker Interception:** `patches/privacy/native-adblock.patch` inspects incoming URIs inside `nsHttpChannel.cpp` and blocks known trackers with `NS_ERROR_BLOCKED_URI`.

### Layer 2: Enterprise `policies.json`
Located in `policies/policies.json`, copied to `browser/app/profile/` during build:
- Disables telemetry, default browser checks, autofill server sync, Firefox accounts, and Pocket.
- Locks `browser.safebrowsing.malware.enabled`, `browser.safebrowsing.phishing.enabled`, and downloads checks.

### Layer 3: `bobzilla.cfg` Autoconfig
Located in `settings/bobzilla.cfg`, loaded as Gecko's global autoconfig:
- Sets `privacy.resistFingerprinting = true` for Day 1 fingerprinting resistance.
- Disables WebRTC IP leakage via `media.peerconnection.ice.no_host = true` and `media.peerconnection.ice.default_address_only = true`.
- Forces Manifest V2 preservation and disables extension manifest v3 enforcement.

### Layer 4: `user.js` Template
Provides standard Arkenfox baseline overrides copied into user profiles on creation.

---

## 2. IPC Socket Configuration Protocol

The Javasandbox subsystem relies on IPC communication between Gecko (`nsProcessBroker.cpp`) and the host `runner-daemon`.

### Socket Resolution Algorithm
Both the C++ client in Gecko and the Rust `runner-daemon` follow the exact same discovery priority:

1. **Environment Variable Override:** `$BOBZILLA_SOCKET` (allows containerized or custom socket binding).
2. **XDG User Runtime:** `$XDG_RUNTIME_DIR/bobzilla-sandbox.sock` (standard Linux desktop runtime path).
3. **Current UID Directory:** `/run/user/<UID>/bobzilla-sandbox.sock`.
4. **Default UID 1000 Directory:** `/run/user/1000/bobzilla-sandbox.sock`.
5. **Fallback Temp Socket:** `/tmp/bobzilla-sandbox-<UID>.sock` or `/tmp/bobzilla-sandbox.sock`.

### IPC Wire Protocol
Communication occurs over a stream-oriented Unix domain socket (`SOCK_STREAM`) with newline-delimited ASCII commands:

| Command | Direction | Response | Description |
|---|---|---|---|
| `PING\n` | Client -> Daemon | `PONG BOBZILLA_DAEMON_OK\n` | Daemon health check |
| `STATUS\n` | Client -> Daemon | `STATUS: READY, ENGINE: ...\n` | Subsystem status inspection |
| `LAUNCH <payload>\n` | Client -> Daemon | `OK LAUNCHED <payload>\n` | Spawns Firecracker MicroVM & FUSE bridge |
| `PAUSE <session>\n` | Client -> Daemon | `OK PAUSED <session>\n` | Suspends MicroVM and dumps state |
| `QUIT\n` or `SHUTDOWN\n` | Client -> Daemon | `OK SHUTTING_DOWN\n` | Graceful daemon process termination |

---

## 3. Mission Control Dashboard Wiring

The Bobzilla Mission Control Dashboard (`patches/ui/bobzilla-dashboard-ui.patch`) injects into `about:preferences#privacy`. It groups the browser's active security posture into three unified cards:

1. **Core Privacy & Anti-Surveillance Engine:** Telemetry, Pocket, Captive Portal, Sponsored Stories, Safe Browsing, and Native Ad-Blocker.
2. **Javasandbox MicroVM Engine:** Runner Daemon Bridge status, Demand-Paged Disk Streamer, and Tab Snapshotting.
3. **Media DRM & Hardware Capabilities:** Widevine L1 Hardware Spoofing, WebGPU Passthrough, and Vertical Tabs / Compact Toolbar workspace.

All entries include descriptive `tooltiptext` attributes explaining the architectural *why* and engine-level behavior to provide intuitive user guidance.
