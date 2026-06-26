# bobzilla

**The browser that doesn't suck.**

A Firefox fork that combines the best of all worlds: Firefox's privacy DNA, Chromium-level DRM support, V8-competitive JavaScript performance, and zero surveillance.

*Part of the bob software ecosystem by Robert Pelloni.*

## Vision

Firefox is the last bastion against Chromium monoculture, but it has problems:
- Netflix/streaming limited to 720p (DRM/Widevine issues)
- JavaScript engine (SpiderMonkey) slower than V8
- Mozilla telemetry and surveillance by default
- Pocket, sponsored tiles, and ad partnerships baked in
- Still the best for ad blocking (Manifest V2 support)
- Still the best for privacy (when configured properly)

**bobzilla** aims to:
1. Fix the DRM situation - full Widevine L1 support for 4K streaming
2. Optimize SpiderMonkey or explore V8 integration paths
3. Strip ALL telemetry and Mozilla services by default
4. Keep Manifest V2 forever - superior ad blocking
5. Bake in privacy settings that LibreWolf/arkenfox provide
6. Maintain Firefox's rendering engine (Gecko) - no Chromium inside
7. Introduce **Javasandbox**: A native hardware-isolated MicroVM architecture to run Java directly within the browser tab.

## Upstream Sources

bobzilla pulls from multiple Firefox forks, cherry-picking the best:

| Fork | What We Take |
|------|--------------|
| **Firefox** | Base browser, Gecko engine, SpiderMonkey |
| **LibreWolf** | Privacy settings, telemetry removal, policies.json |
| **Floorp** | UI customization, vertical tabs, workspaces |
| **Tor Browser** | Fingerprinting resistance patches |
| **Waterfox** | Legacy extension support insights |

## Key Features (Status)

### Privacy (Day 1)
- [x] All telemetry disabled by default
- [x] No Mozilla accounts integration
- [x] No Pocket
- [x] No sponsored content
- [x] Enhanced tracking protection on strict
- [x] Fingerprinting resistance enabled
- [x] HTTPS-only mode default
- [x] Google Safe Browsing APIs disabled

### Performance & Experimental
- [x] V8-Spider Integration Design
- [x] Javasandbox MicroVM Runner-Daemon scaffolded (Rust/Tokio)
- [ ] SpiderMonkey optimizations
- [ ] Memory usage improvements
- [ ] Startup time optimization

### DRM/Streaming
- [x] Widevine L1 hardware spoofing patches drafted
- [ ] Netflix 1080p/4K support verified
- [ ] Hardware video decoding improvements

### Ad Blocking
- [x] Manifest V2 preserved forever
- [x] Native network-level ad-blocking hook drafted
- [ ] Built-in uBlock Origin (optional)

### UI/UX
- [x] Vertical tabs mock patches
- [x] Floorp-style compact URL bar mock patches
- [ ] Workspaces/tab groups
- [ ] Clean, minimal default theme

## Building

### Prerequisites
- Python 3.x
- Rust (latest stable)
- Visual Studio 2022 (Windows) or GCC/Clang (Linux/Mac)
- `git-cinnabar` (Required for fetching Mozilla sources)
- ~50GB disk space
- ~16GB RAM recommended

### Quick Start

```bash
# Clone the repo
git clone https://github.com/user/bobzilla.git
cd bobzilla

# Bootstrap the build environment & submodules
./scripts/fetch-source.sh
./mach bootstrap

# Configure the build
cp mozconfig.bobzilla mozconfig

# Build
./scripts/build.sh

# Run
cd mozilla-unified && ./mach run
```

## Repository Structure

```
bobzilla/
├── guest-os/            # Buildroot configurations for the Javasandbox MicroVM
├── mozilla-unified/     # Firefox source (submodule)
├── patches/             # Our modifications
│   ├── privacy/         # Telemetry removal, tracking protection
│   ├── drm/             # Widevine/DRM improvements
│   ├── performance/     # SpiderMonkey optimizations
│   ├── ui/              # UI customizations
│   └── branding/        # bobzilla branding
├── runner-daemon/       # Rust IPC controller for Javasandbox MicroVMs
├── settings/            # Default preferences (like LibreWolf)
├── policies/            # Enterprise policies
├── scripts/             # Build and maintenance scripts
└── docs/                # Architecture documentation
```

## Philosophy

1. **Privacy by default** - No opt-out, just private
2. **Performance matters** - Compete with Chromium
3. **DRM pragmatism** - We hate DRM but Netflix exists
4. **Community first** - No corporate surveillance
5. **Absurdist branding** - It's called bobzilla, embrace it

## License

MPL 2.0 (inherited from Firefox)

## Related Projects

- [bobcoin](https://github.com/robertpelloni/bobcoin) - Proof-of-play cryptocurrency
- [bobfilez](https://github.com/robertpelloni/bobfilez) - File organization engine
- [FWBer](https://github.com/robertpelloni/fwber) - Dating platform
- [bob's game](https://bobsgame.com) - The game that started it all

---

*"All browsers will eventually be bobzilla running boblang on bobos."*
