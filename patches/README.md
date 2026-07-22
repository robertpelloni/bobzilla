# bobzilla patches

This directory contains the patches applied to the `mozilla-unified` Firefox source code to create bobzilla. We follow a strict patch-based workflow to avoid modifying the upstream source directly.

## Directory Structure

*   `privacy/`: Patches that enforce strict privacy features, such as completely removing telemetry (`0001-disable-telemetry.patch`), safe browsing checks, sync capabilities, and pocket integration.
*   `drm/`: Patches related to Widevine and DRM capability spoofing.
*   `performance/`: Patches for memory tuning and SpiderMonkey optimizations.
*   `ui/`: Patches for UI adjustments like vertical tabs and compact toolbars.
*   `branding/`: Patches to replace Mozilla branding with bobzilla branding.

## Important Notes

*   Patches are applied via `scripts/apply-patches.sh` during the build process.
*   Always ensure patches are atomic and well-documented.
