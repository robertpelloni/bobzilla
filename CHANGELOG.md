# CHANGELOG

## [0.1.3] - 2026-01-12
### Changed
- Explicitly enabled `privacy.resistFingerprinting` in `settings/bobzilla.cfg` to fulfill the final outstanding item on the "Privacy (Day 1)" milestone.
- Verified that all other zero-telemetry, Pocket-disabling, and HTTPS-only default configurations were already correctly implemented.

## [0.1.2] - 2026-01-12
### Added
- Created `docs/V8_SPIDER_DISPATCHER.md` to define the architectural design and implementation phases for the "V8-Spider Hybrid Engine" milestone, fulfilling the first critical goal from `ROADMAP.md`.

## [0.1.1] - 2026-01-12
### Added
- Extracted and formalized project roadmap into `ROADMAP.md` based on initial architectural and product visions.
- Extracted immediate, granular tasks into `TODO.md` focusing on day-1 privacy, performance, DRM pragmatism, and ad blocking.
- Created `VISION.md` to establish the ultimate goal of Bobzilla as an autonomous "Agent Shell" with strict privacy by default.
- Established `MEMORY.md` to document internal architectural traits, focusing on the patch-based system and strict telemetry disabled defaults.
- Migrated and expanded deployment steps into `DEPLOY.md` to provide a robust build guide for Windows, Linux, and macOS.
- Appended aggressive new concepts to `IDEAS.md` including Rust-first subcomponents, No-Chrome interfaces, and local SLM ad-nuking.
- Initialized global `VERSION.md` with version 0.1.1 to conform to Robert Pelloni Omni-Workspace versioning mandates.

### Rationale
To comply with the PRINCIPLE DIRECTIVE of the Robert Pelloni monorepo, a comprehensive project state analysis was performed. Foundational documentation was missing or disorganized. Creating these standard governance documents ensures that future autonomous agents (Claude, GPT, Gemini) have a clear, centralized understanding of the project's vision, structure, current state, and deployment procedures.
