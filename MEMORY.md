# MEMORY

## Internal Architectural Observations

### Codebase Structure
- **Patch-Based System:** The project intentionally avoids direct modifications to the Firefox source. Instead, it relies on atomic, well-documented patches organized into `patches/privacy`, `patches/drm`, `patches/performance`, `patches/ui`, and `patches/branding`.
- **Settings Overlay:** Core behavioral modifications are applied via a settings overlay utilizing `policies.json` and a LibreWolf-inspired `user.js` (or `bobzilla.cfg`).
- **Submodule Dependency:** The massive Firefox codebase (~3GB) is maintained as a submodule at `mozilla-unified`. AI agents must not attempt to index or parse the entire submodule contextually; focus remains on the wrapper, patch scripts, and build configurations.

### Build & Tooling Ecosystem
- **Build Orchestration:** Uses Mozilla's `mach` orchestrator, wrapper scripts (`scripts/build.sh`), and standard `mozconfig` configurations customized for Bobzilla.
- **Automation Scripts:**
  - `./scripts/fetch-source.sh` ensures the submodule is correctly populated.
  - `./scripts/apply-patches.sh` sequentially applies layered patches.

## Codebase Traits & Design Preferences
- **Zero Telemetry Toleration:** Every instance of telemetry, health reporting, and remote experimentation is aggressively stripped or disabled at the configuration level.
- **Preference for Strict Privacy:** `privacy.trackingprotection.enabled` is forced to `true`, and network predictions/prefetching are disabled.
- **Pragmatic Compromise on Media:** While LibreWolf might disable DRM by default, Bobzilla actively *enables* `media.eme.enabled` and hardware acceleration to support the user-satisfaction design concerning media playback.
- **Omni-Workspace Alignment:** This repository is part of the larger Robert Pelloni monorepo structure. It must maintain strict adherence to versioning rules, updating `CHANGELOG.md`, `VERSION.md`, and documenting agent handoffs in `HANDOFF.md`.
