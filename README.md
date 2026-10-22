# bobzilla

A Firefox fork project: Firefox's Gecko engine and Manifest V2 extension support, with Mozilla's telemetry/services stripped out, privacy hardening baked in by default, and DRM (Widevine) kept working for streaming. Part of Robert Pelloni's bob* software ecosystem.

## What this repo actually is

This is **not** a full browser source tree. It is the fork's *configuration and patch layer*:

| Path | Contents |
|---|---|
| `mozilla-unified/` | The upstream Firefox tree (submodule) the fork builds on |
| `mozconfig.bobzilla` | The build configuration |
| `patches/` | Patches applied over upstream |
| `policies/`, `settings/` | Enterprise policies and default preferences (the privacy hardening) |
| `guest-os/`, `runner-daemon/`, `scripts/` | Build/run tooling |

## Goals (from the project's vision doc)

1. Full Widevine support for HD/4K streaming (the stock-Firefox DRM limitation is the original motivation)
2. All telemetry and Mozilla services removed or disabled by default
3. Manifest V2 kept forever — superior ad blocking depends on it
4. LibreWolf/arkenfox-style privacy defaults baked in, plus Tor Browser fingerprinting-resistance patches where they fit
5. UI customization cherry-picked from Floorp (vertical tabs, workspaces)

The previous README presented a feature checklist with most items ticked. Treat those as goals, not verified state: the honest summary is that the patch/policy layer exists and the vision is documented (`VISION.md`, `ROADMAP.md`, `TODO.md`); a polished, released bobzilla build is not published from this repo.

## Status

Experimental, single-developer. If you want a privacy-hardened Firefox today, the upstream projects this fork draws from (LibreWolf, Floorp, Waterfox) ship releases; bobzilla is the workbench, not the product.
