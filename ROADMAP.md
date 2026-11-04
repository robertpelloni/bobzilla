# ROADMAP: bobzilla

## Phase 1: Foundation (Current)
- Submodule integration and shallow clone mechanics.
- Baseline configuration overrides (`settings/bobzilla.cfg`, `policies/policies.json`).
- Rust host runner daemon for Javasandbox Firecracker MicroVMs with dynamic socket resolution and IPC CLI controls.
- Patch verification pipeline and structural test automations.

## Phase 2: Privacy Hardening & Core Architecture
- Source-level removal of Pocket, Telemetry, Mozilla Accounts Sync, and sponsored feeds.
- Hardcoded Google Safe Browsing elimination to ensure complete local URL resolution privacy.
- Native C++ network interception for tracker/ad domain filtering.
- Consolidated Bobzilla Mission Control Dashboard in `about:preferences#privacy`.

## Phase 3: Features & System Integration
- Vertical tabs and compact navigation toolbar styling.
- Custom UI density and display scaling overrides.
- WebRequest API preservation (permanent Manifest V2 support).
- End-to-end Javasandbox MicroVM tab lifecycle (tab open -> socket dispatch -> Firecracker boot -> snapshot on tab blur/close).
- Desktop system tray companion applet with daemon health telemetry and graceful termination.
