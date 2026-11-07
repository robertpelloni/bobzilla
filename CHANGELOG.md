# Changelog

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
