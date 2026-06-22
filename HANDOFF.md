# Handoff Document

## Session Summary
- Analyzed `AGENTS.md`, `README.md`, and global LLM instructions.
- Enforced Day 1 Privacy tasks in `settings/bobzilla.cfg` (enabled fingerprinting resistance).
- Noted that `mozilla-unified` submodule is missing/uninitialized, blocking C++ patch implementation.
- Received user request to design an architecture for running native Java inside a browser tab using a full JDK stack and Linux VM.
- Authored the comprehensive architectural design for this `javasandbox` system, mapping out the Firecracker MicroVM, Firefox fork hooks, snapshot-on-close logic, and the Monorepo structure. Saved to `docs/architecture/javasandbox-architecture.md`.

## Next Steps
- Initialize the `mozilla-unified` submodule using `scripts/fetch-source.sh`.
- Review the newly added `javasandbox` architecture document for potential integration paths or next steps in establishing the monorepo structure it defines.
