# Handoff Document

## Session Summary
- Fixed a severe regression in `scripts/build.sh` identified by the code review. The script now properly retains the `exit 1` structural integrity if the `mozilla-unified` submodule is missing, ensuring the build fails loudly in CI/CD rather than falsely succeeding.
- Updated the global version tag to `0.1.21` per Omni-Workspace protocols.

## Next Steps
- The primary blocking constraint remains the uninitialized `mozilla-unified` 3GB submodule.
