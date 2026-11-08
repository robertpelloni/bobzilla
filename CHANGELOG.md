# Changelog

## [0.1.80] - 2026-11-05
### Changed
- Added `@media (forced-colors: active)` high-contrast accessibility styles in `patches/ui/custom-layout.patch` with `CanvasText` boundaries, `Highlight` selections, and pinned tab dividers.

## [0.1.79] - 2026-11-05
### Changed
- Added `@media (prefers-reduced-motion: reduce)` accessibility overrides in `patches/ui/custom-layout.patch` to disable tab hover translations, scale feedback, and pulse animations for users with motion sensitivity.

## [0.1.78] - 2026-11-05
### Changed
- Enhanced `patches/ui/custom-layout.patch` with neon glow drop indicators (`box-shadow: 0 -2px 8px rgba(53, 132, 228, 0.6)`), Wayland/macOS subpixel anti-aliasing easing, and pinned tabs separator divider styling.

## [0.1.77] - 2026-11-05
### Changed
- Hardened Pocket integration bypass in `patches/privacy/remove-pocket.patch` with immediate Promise rejection/resolution for save and article retrieval endpoints.

## [0.1.76] - 2026-11-05
### Changed
- Refined tracking protection UI in `patches/ui/tracking-protection-ui.patch` to advertise active native C++ network ad-blocking and excised telemetry indicators.

## [0.1.75] - 2026-11-05
### Changed
- Enhanced `about:support` troubleshooting diagnostics (`patches/ui/about-support.patch`) to report `bobzillaDiagnostics` (Javasandbox IPC status, VirtIO-GPU DMA-BUF passthrough, Widevine L1 spoofing, and Jemalloc memory tuning).

## [0.1.74] - 2026-11-05
### Changed
- Added split-view / multi-selected tab border styling (`.tabbrowser-tab[multiselected="true"]`) in `patches/ui/custom-layout.patch`.
- Added drag-over ghosting styling (`.tabbrowser-tab[dragging="true"]`) with dashed boundaries and dimmed opacity.
- Enhanced vertical tab scrollbar thumb contrast on container hover.

## [0.1.73] - 2026-11-05
### Changed
- Enhanced `patches/performance/oom-tuning.patch` with active jemalloc mallctl tuning: setting dirty and muzzy page decay to 0ms and invoking idle arena purging for aggressive background RAM reclamation.

## [0.1.72] - 2026-11-05
### Changed
- Enhanced `patches/drm/widevine-l1.patch` with HDCP 2.2 advertising and hardware-secure video decoding robustness (`HW_SECURE_ALL`) for 4K streaming.

## [0.1.71] - 2026-11-05
### Changed
- Expanded `patches/performance/webgpu-passthrough.patch` in `dom/canvas/WebGLContext.cpp` to integrate zero-copy VirtIO-GPU dma-buf surface synchronization and virtqueue flushing.

## [0.1.70] - 2026-11-05
### Added
- Added unread numerical badge counters (`.tab-badge`) for vertical tabs in `patches/ui/custom-layout.patch`.
- Added inline loading progress bar indicators (`.tab-progress-bar`) with smooth gradient animations for loading tabs (`[busy="true"]`).

## [0.1.69] - 2026-11-05
### Changed
- Added high-visibility keyboard focus styling (`:focus-visible`) for tabs and action buttons in `patches/ui/custom-layout.patch`.
- Added tab context menu active state feedback (`.tabbrowser-tab[contextmenu-active="true"]`).
- Added vertical tab scroll padding (`scroll-padding-top: 8px; scroll-padding-bottom: 8px`) for smooth tab list centering.

## [0.1.68] - 2026-11-05
### Changed
- Improved error handling in `scripts/validate-patches.sh` using standard `exit 1`/`exit 0` return codes and explicit reporting of any malformed patches.

## [0.1.67] - 2026-11-05
### Added
- Enhanced `scripts/bobzilla-tray.py` with an interactive live terminal dashboard mode (`--dashboard`), latency profiling, and ANSI terminal refresh.
- Added discovery resolution assertions and telemetry output validation to `bobzilla-tray.py --test`.

## [0.1.66] - 2026-11-05
### Changed
- Added visual styling for discarded and unloaded tabs (`.tabbrowser-tab[pending="true"]`, `.tabbrowser-tab[discarded="true"]`) with dimmed opacity (0.72) in `patches/ui/custom-layout.patch`.
- Added sound playing pulse animation (`bobzilla-pulse-sound`) for `.tab-icon-sound[soundplaying="true"]`.
- Refined icon scaling (14px x 14px) and spacing in compact density mode (`browser.uidensity == 1`).

## [0.1.65] - 2026-11-05
### Changed
- Added tab container contextual identity badges (Multi-Account Containers color indicators) in `patches/ui/custom-layout.patch`.
- Added subtle active tab ambient glow (`box-shadow: 0 0 10px rgba(53, 132, 228, 0.25)`).
- Added tab group header styling and collapse chevron animations (`rotate(-90deg)`).

## [0.1.64] - 2026-11-05
### Changed
- Added smooth tab closing exit animations (`opacity: 0; transform: translateX(-8px)`) in `patches/ui/custom-layout.patch`.
- Refined light theme contrast styling for `.bobzilla-vertical-tabs` and selected tabs.
- Added high-DPI display font-smoothing overrides (`-webkit-font-smoothing: antialiased`).

## [0.1.63] - 2026-11-05
### Changed
- Refined vertical tab transitions and micro-animations in `patches/ui/custom-layout.patch` using GPU-accelerated cubic-bezier curves (`cubic-bezier(0.16, 1, 0.3, 1)`).
- Added `contain: paint layout` to `.bobzilla-vertical-tabs` to eliminate layout jitter and repainting overhead during width transitions.
- Added smooth tab hover micro-elevation (`transform: translateX(2px)`) and scale feedback on action buttons.

## [0.1.62] - 2026-11-05
### Changed
- Added sidebar collapse/expand toggle controls (`sidebar.verticalTabs.collapsed`) in `patches/ui/vertical-tabs.patch`.
- Added header action bar in `patches/ui/custom-layout.patch` with dedicated compact "New Tab" button and sidebar collapse toggle button.
- Refined tab loading spinner (throbber) positioning and alignment in vertical tab strip.

## [0.1.61] - 2026-11-05
### Changed
- Enhanced `patches/ui/custom-layout.patch` with audio playing/mute indicators for vertical tabs, allowing one-click muting without horizontal tab expansion.
- Added attention and unread activity notification dot badges on background tabs in vertical orientation (`::after` pseudo-element with subtle glow).
- Customized vertical tab container scrollbar with ultra-slim modern scrollbar geometry (`scrollbar-width: thin`).

## [0.1.60] - 2026-11-05
### Changed
- Refined and enhanced dynamic vertical tabs in `patches/ui/custom-layout.patch` and `patches/ui/vertical-tabs.patch`.
- Added responsive resize viewport observer in `tabbrowser.js` adjusting vertical tab width to 180px on compact screens/split-screen layouts (< 960px).
- Arranged pinned tabs horizontally as compact icon-only tiles (38px x 34px) in vertical mode to save vertical screen space.
- Added visual drag-and-drop reordering indicators (`border-top: 2px solid #3584e4`) and tab hover feedback transitions.

## [0.1.59] - 2026-11-05
### Changed
- Enhanced `patches/ui/custom-layout.patch` in `browser.css` with hardware-accelerated transitions (`will-change: width, box-shadow`), text label opacity fades on auto-hover expand, tab border-radius smoothing, and interactive close button hover states for vertical tabs.

## [0.1.58] - 2026-11-05
### Changed
- Refined `patches/ui/custom-layout.patch` in `browser.css` to implement collapsed icon-only sidebar mode (`bobzilla-sidebar-collapsed="true"` at 48px width) with smooth auto-hover expansion to 240px and active tab accent styling (`border-left: 3px solid #3584e4`).

## [0.1.57] - 2026-11-05
### Changed
- Enhanced `patches/ui/compact-urlbar.patch` in `UrlbarInput.sys.mjs` to suppress disruptive megabar breakout expansion during focus (`bobzilla-no-megabar-expand`) when compact density mode is active.

## [0.1.56] - 2026-11-05
### Changed
- Refined `patches/ui/custom-layout.patch` to scope `#TabsToolbar` hiding strictly to `:root[bobzilla-vertical-tabs="true"]`, preserve horizontal layout fallback when vertical tabs are toggled off, and provide CSS transitions, sidebar border boundaries, and compact vertical tab sizing.

## [0.1.55] - 2026-11-05
### Changed
- Integrated `browser.uidensity` preference into `patches/ui/vertical-tabs.patch` (`tabbrowser.js`), dynamically styling the vertical tab list for compact mode (`density="compact"`, `bobzilla-compact-tabs`) when `browser.uidensity == 1` and scaling for touch mode without requiring browser restarts.

## [0.1.54] - 2026-11-05
### Changed
- Expanded `patches/ui/custom-scaling.patch` to surface Compact Mode in UI customize settings (`browser.compactmode.show = true`), set compact density by default (`browser.uidensity = 1`), enforce base display scaling (`layout.css.devPixelsPerPx = 1.0`), and enable sub-pixel text anti-aliasing.

## [0.1.53] - 2026-11-05
### Changed
- Expanded `patches/ui/vertical-tabs.patch` to dynamically bind to the `sidebar.verticalTabs` preference, auto-updating tab strip orientation and root document layout attributes for Floorp/Zen-style vertical tab workflows.

## [0.1.52] - 2026-11-05
### Added
- Authored comprehensive end-to-end user and developer guide `docs/JAVASANDBOX_GUIDE.md` covering URL scheme routing, FUSE streaming, MicroVM execution, daemon management, desktop system tray operations, socket discovery, and guest OS bundle packaging.
- Expanded automated integration test suite in `runner-daemon/src/main.rs` to 9 tests, adding edge-case URI parsing validation, buffer boundary checks, and concurrent multi-client IPC connection load testing.

## [0.1.51] - 2026-11-04
### Changed
- Hardened `settings/bobzilla.cfg` by locking Manifest V2 preferences with `lockPref` to permanently prevent runtime or experiment overrides (`extensions.manifestV3.enabled`, `extensions.legacy.enabled`).
- Removed synthetic domain restrictions on web extensions via `lockPref("extensions.webextensions.restrictedDomains", "")`, guaranteeing unrestricted ad-blocking across all sites for uBlock Origin.
- Disabled automatic extension quarantine via `lockPref("extensions.quarantine.enabled", false)`.

## [0.1.50] - 2026-11-04
### Added
- Created `scripts/bobzilla-tray.py`, providing a desktop system tray companion and IPC monitor for the Javasandbox runner daemon. Includes real-time subsystem telemetry, pinging, formatted mission control output, and graceful quit/shutdown capabilities with automated `--test` runner support.

## [0.1.49] - 2026-11-04
### Added
- Implemented comprehensive automated end-to-end integration tests in `runner-daemon/src/main.rs`, testing asynchronous tab IPC launches, URI protocol routing, tab background state serialization via `SnapshotManager`, and socket lifecycle termination.
- Expanded native ad-blocker domain filtering in `patches/privacy/native-adblock.patch` (`nsHttpChannel.cpp`), adding native C++ interception for 12 major tracker and analytics networks (Google Tag Manager, Criteo, AppNexus, Outbrain, Taboola, Facebook Tracker, and background Mozilla telemetry hosts).

## [0.1.48] - 2026-11-04
### Added
- Authored comprehensive architecture documentation `docs/architecture/config-protocol.md` defining the 5-layer configuration hierarchy, IPC socket discovery algorithm, Unix socket ASCII wire protocol, and Mission Control Dashboard preference wiring.
- Formally concluded Phase 1 (Foundation) in `ROADMAP.md` and detailed immediate short-term milestones for Phase 2 (Privacy Hardening & Network Interception) and Phase 3 (UI & Virtualization Integration).
- Updated `TODO.md` with targeted action items for live Gecko tab testing of the Javasandbox protocol and native network domain filtering list expansions.

## [0.1.47] - 2026-11-04
### Added
- Implemented robust dynamic multi-path IPC socket discovery in `runner-daemon/src/main.rs`, resolving paths through `$BOBZILLA_SOCKET`, `$XDG_RUNTIME_DIR`, `/run/user/$UID`, `/run/user/1000`, and `/tmp` fallbacks.
- Integrated full CLI daemon lifecycle management in `runner-daemon`, enabling direct querying and control (`ping`, `status`, `quit`/`shutdown`) for background operations and system tray tooling.
- Added comprehensive unit tests in `runner-daemon` validating socket resolution, ping responses, status checks, and microvm launch commands.
- Updated `patches/javasandbox-ipc-client.patch` to use multi-path socket discovery in `nsProcessBroker.cpp`, ensuring native Gecko hooks reach the runner-daemon regardless of container or desktop UID layout.
- Consolidated and redesigned the Bobzilla Mission Control Dashboard in `patches/ui/bobzilla-dashboard-ui.patch` (`about:preferences#privacy`), categorizing settings into Core Privacy & Anti-Surveillance, Javasandbox MicroVM Engine, and Media DRM & Hardware Acceleration, with rich explanatory tooltips and distinct engine-level status indicators.
- Upgraded `scripts/validate-patches.sh` to perform thorough structural verification and dry-run execution against Firefox source.
- Created `/home/jules/self_created_tools/patch_inspector.py` to automate hunk and header validation across all patch files.
- Configured clean `.gitignore` to prevent Rust compilation artifacts and object directories from polluting the tree.
