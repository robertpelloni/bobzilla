# bobzilla Technical Research

## Firefox Forks Analysis

### Floorp (github.com/Floorp-Projects/Floorp)
- **Origin**: Japan, 7.3k GitHub stars
- **Base**: Firefox Release (not ESR)
- **Key Features**: Vertical tabs (now native in Firefox 140+), multi-row tabs, workspaces, heavy UI customization
- **Approach**: Direct modifications to Firefox source
- **Docs**: docs.floorp.app

### LibreWolf (gitlab.com/librewolf-community/browser/source)
- **Origin**: Community privacy project
- **Base**: Firefox ESR (more stable)
- **Key Features**: All telemetry stripped, no Pocket, no Mozilla services, hardened settings
- **Approach**: Patches + policies.json + librewolf.cfg
- **Settings repo**: codeberg.org/librewolf/settings
- **What we take**: Privacy settings, telemetry removal approach, policies.json template

### Waterfox
- **Status**: Owned by System1 (ad company) since 2020 - trust concerns
- **Base**: Firefox ESR
- **Key Features**: Legacy extension support (XUL), classic UI options
- **What we take**: Possibly legacy extension insights, but be wary of ownership

### Tor Browser
- **Base**: Firefox ESR
- **Key Features**: Maximum anonymity, fingerprinting resistance, NoScript bundled
- **Approach**: Extensive patches for privacy/anonymity
- **What we take**: Fingerprinting resistance patches (selectively, as they break sites)

---

## DRM / Widevine Analysis

### The Problem
- Firefox is stuck at **Widevine L3** (software-only decryption)
- Netflix limits L3 to **720p** max
- Chromium browsers get **Widevine L1** (hardware-backed) = 1080p/4K

### Why Firefox Can't Just "Enable" L1
1. **L1 requires hardware TEE** (Trusted Execution Environment)
2. **Device must be certified by Google** - not browser's choice
3. **It's per-device, not per-browser** - same hardware with Chrome vs Firefox gets different treatment
4. **Google controls certification** - conflict of interest with Chrome

### Technical Reality
- Widevine CDM is a **black box binary** from Google
- L1 certification requires:
  - Hardware security module (TPM/TEE)
  - Google audit and approval
  - Per-device key provisioning
- Firefox on same hardware as Chrome should theoretically support L1, but Google's licensing restricts it

### What bobzilla Can Do
1. **Ensure Widevine is enabled and working** (L3 at minimum)
2. **Optimize video pipeline** for best possible quality at L3
3. **Advocate/document** the anti-competitive nature of Google's DRM gatekeeping
4. **Monitor** for any changes in Widevine licensing
5. **Hardware acceleration** - ensure VAAPI/NVDEC work properly

### Realistic Expectation
**We cannot fix the 720p Netflix limit** - it's Google's gatekeeping, not a technical limitation we can patch. This is a policy/antitrust issue, not a code issue.

---

## JavaScript Engine: SpiderMonkey vs V8

### Current Performance Gap
| Benchmark | Chrome (V8) | Firefox (SpiderMonkey) | Gap |
|-----------|-------------|------------------------|-----|
| Speedometer 3.0 | 37.8 | 34.6 | ~10% |
| Synthetic (JetStream) | - | - | 15-40% |
| Memory Usage | Higher | 8-30% lower | Firefox wins |

### SpiderMonkey Architecture
```
JavaScript → Parser → Bytecode → Baseline JIT → CacheIR → WarpBuilder → IonMonkey (optimizing JIT)
```

### Recent SpiderMonkey Improvements
- **Warp compiler** (2020): 20% improvement on Google Docs
- **Wasm compilation**: 75x speedup (October 2024)
- **CacheIR**: Better inline caching
- Gap is **closing** - real-world difference now ~10%

### Can We Embed V8?
**Theoretically possible, practically insane:**
- Different garbage collectors
- Different object models
- Different DOM bindings (V8 uses Blink's, SpiderMonkey uses Gecko's)
- Would essentially require rewriting the browser
- Firefox tried this concept (Servo) - different approach

### bobzilla JS Strategy
1. **Track upstream SpiderMonkey improvements** - Mozilla is actively working on this
2. **Enable all performance flags** in mozconfig (LTO, PGO)
3. **Contribute benchmark data** to identify bottlenecks
4. **Don't chase V8** - focus on real-world performance, not synthetic benchmarks
5. **Memory efficiency** is a Firefox strength - emphasize it

---

## Manifest V2 vs V3 (Ad Blocking)

### What Google Changed in Manifest V3
- Replaced **webRequest API** (blocking) with **declarativeNetRequest**
- declarativeNetRequest has **rule limits** (was 5k, now 30k, still limiting)
- **Cannot dynamically modify requests** - must pre-declare rules
- **Breaks cosmetic filtering** and advanced blocking

### Why This Hurts Ad Blocking
- uBlock Origin uses webRequest to:
  - Block requests **before they're made**
  - Dynamically update filter lists
  - Apply cosmetic filters
  - Handle anti-adblock scripts
- MV3's declarativeNetRequest can't do this effectively

### Firefox's Stance
- **Supports both MV2 and MV3**
- **Preserves webRequest blocking capability** in MV3
- Extensions like uBlock Origin work **fully** on Firefox

### bobzilla Ad Blocking Strategy
1. **Keep MV2 support forever** (or until heat death of universe)
2. **Preserve full webRequest API** - no deprecation
3. **Consider built-in blocking** (like Brave) as optional feature
4. **DNS-level blocking** integration (optional)

---

## Privacy Concerns with Firefox/Mozilla

### What Mozilla Collects (by default)
- Telemetry (usage data, performance metrics)
- Crash reports
- Health reports
- Studies/experiments (Normandy)
- Pocket recommendations
- Sponsored tiles on new tab
- Firefox Suggest (sponsored suggestions)

### Controversial Mozilla Decisions
- **Pocket acquisition** and integration
- **Sponsored content** in new tab page
- **Partnership with advertising companies**
- **AI features** with unclear data handling
- **Mr. Robot extension** installed without consent (2017)

### What LibreWolf Disables
Everything above, plus:
- Firefox accounts
- Sync (can be re-enabled)
- DRM (user choice in bobzilla - we enable it)
- Safe Browsing (Google service) - controversial
- Various about:config privacy settings

### bobzilla Privacy Approach
1. **All telemetry OFF by default** (not opt-out, just off)
2. **No Pocket**
3. **No sponsored content anywhere**
4. **No Mozilla accounts by default** (can enable)
5. **Keep Safe Browsing** (privacy tradeoff worth it for security)
6. **Keep DRM** (pragmatic - people want Netflix)
7. **Strict tracking protection default**
8. **arkenfox-inspired user.js** as default

---

## Build System Notes

### Firefox Build Requirements
- **Disk**: ~50GB for source + build
- **RAM**: 16GB+ recommended (32GB for parallel builds)
- **Time**: 1-4 hours depending on hardware
- **Tools**: Python 3.x, Rust, Visual Studio 2022 (Windows) or GCC/Clang

### Mozilla Source Control
- **Primary**: Mercurial (hg.mozilla.org/mozilla-unified)
- **GitHub mirror**: github.com/nicedoc/nicedoc (unofficial)
- bobzilla will use **Git** with Mozilla source as submodule

### Build Configuration
- **mozconfig**: Main build configuration
- **moz.build**: Per-directory build definitions  
- **mach**: Python-based build orchestrator

### Our Approach
1. **Track Firefox ESR** (like LibreWolf) for stability
2. **Maintain patches** rather than forking source directly
3. **Automate rebasing** when new ESR releases
4. **CI/CD** for multi-platform builds

---

## Roadmap

### Phase 1: Foundation (Current)
- [x] Create repo structure
- [x] Document research
- [x] Create initial configs (mozconfig, policies.json, settings)
- [ ] Add Firefox source as submodule
- [ ] First successful build

### Phase 2: Privacy Hardening
- [ ] Apply LibreWolf-style privacy patches
- [ ] Strip telemetry
- [ ] Remove Pocket
- [ ] Remove sponsored content
- [ ] Custom branding

### Phase 3: Features
- [ ] Vertical tabs (native Firefox 140+)
- [ ] UI customization options
- [ ] Built-in ad blocking (optional)
- [ ] Performance tuning

### Phase 4: Distribution
- [ ] Windows installer
- [ ] Linux packages (AppImage, Flatpak, deb, rpm)
- [ ] macOS dmg
- [ ] Auto-update system (not Mozilla's)

---

## References

- Firefox Source Docs: https://firefox-source-docs.mozilla.org/
- LibreWolf: https://librewolf.net/
- Floorp: https://floorp.app/
- arkenfox user.js: https://github.com/arkenfox/user.js
- Mozilla Build Instructions: https://firefox-source-docs.mozilla.org/setup/
