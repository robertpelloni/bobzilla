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

## Upstream Sources

bobzilla pulls from multiple Firefox forks, cherry-picking the best:

| Fork | What We Take |
|------|--------------|
| **Firefox** | Base browser, Gecko engine, SpiderMonkey |
| **LibreWolf** | Privacy settings, telemetry removal, policies.json |
| **Floorp** | UI customization, vertical tabs, workspaces |
| **Tor Browser** | Fingerprinting resistance patches |
| **Waterfox** | Legacy extension support insights |

## Key Features (Planned)

### Privacy (Day 1)
- [ ] All telemetry disabled by default
- [ ] No Mozilla accounts integration
- [ ] No Pocket
- [ ] No sponsored content
- [ ] Enhanced tracking protection on strict
- [ ] Fingerprinting resistance enabled
- [ ] HTTPS-only mode default

### Performance
- [ ] SpiderMonkey optimizations
- [ ] Investigate V8 embedding (research)
- [ ] Memory usage improvements
- [ ] Startup time optimization

### DRM/Streaming
- [ ] Widevine L1 investigation
- [ ] Netflix 1080p/4K support
- [ ] Hardware video decoding improvements

### Ad Blocking
- [ ] Manifest V2 preserved forever
- [ ] webRequest API fully supported
- [ ] Built-in uBlock Origin (optional)
- [ ] Network-level blocking capabilities

### UI/UX
- [ ] Floorp-style customization
- [ ] Vertical tabs built-in
- [ ] Workspaces/tab groups
- [ ] Clean, minimal default theme

## Building

### Prerequisites
- Python 3.x
- Rust (latest stable)
- Visual Studio 2022 (Windows) or GCC/Clang (Linux/Mac)
- ~50GB disk space
- ~16GB RAM recommended

### Quick Start

```bash
# Clone the repo
git clone https://github.com/user/bobzilla.git
cd bobzilla

# Bootstrap the build environment
./mach bootstrap

# Configure the build
cp mozconfig.bobzilla mozconfig

# Build
./mach build

# Run
./mach run
```

## Repository Structure

```
bobzilla/
├── mozilla-unified/     # Firefox source (submodule)
├── patches/             # Our modifications
│   ├── privacy/         # Telemetry removal, tracking protection
│   ├── drm/             # Widevine/DRM improvements
│   ├── performance/     # SpiderMonkey optimizations
│   ├── ui/              # UI customizations
│   └── branding/        # bobzilla branding
├── settings/            # Default preferences (like LibreWolf)
├── policies/            # Enterprise policies
├── scripts/             # Build and maintenance scripts
└── docs/                # Documentation
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
