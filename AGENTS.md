# AGENTS.md

**Generated:** 2026-01-12 | **Branch:** main | **Version:** 0.1.0

## Overview

**bobzilla** - Firefox fork aiming for privacy-by-default, Chromium-competitive performance, and proper DRM support. Part of the bob software ecosystem.

## Structure

```
bobzilla/
├── README.md            # Project overview and vision
├── AGENTS.md            # This file
├── mozilla-unified/     # Firefox source (submodule, ~3GB)
├── patches/             # Patch files organized by category
├── settings/            # Default preferences (librewolf-style)
├── policies/            # Enterprise policies
├── scripts/             # Build automation
└── docs/                # Technical documentation
```

## Build System

Firefox uses `mach` (Python-based build orchestrator) with:
- **moz.build** files for build definitions
- **mozconfig** for build configuration
- Mercurial (hg) for Mozilla's repos, Git for our fork

## Key Concepts

1. **Patch-based approach** - Don't modify Firefox source directly; maintain patches
2. **Upstream tracking** - Rebase patches on Firefox ESR releases
3. **Settings overlay** - Use policies.json and user.js for preferences
4. **Branding separation** - Keep branding in separate directory

## Upstream Repos

| Source | URL | Purpose |
|--------|-----|---------|
| Firefox | hg.mozilla.org/mozilla-unified | Base browser |
| LibreWolf | gitlab.com/librewolf-community | Privacy patches |
| Floorp | github.com/Floorp-Projects/Floorp | UI features |
| arkenfox | github.com/arkenfox/user.js | user.js template |

## Development Workflow

1. Clone with submodules: `git clone --recursive`
2. Apply patches: `./scripts/apply-patches.sh`
3. Configure: `cp mozconfig.bobzilla mozconfig`
4. Build: `./mach build`
5. Test: `./mach run`

## LLM Instructions

- Firefox source is HUGE (~3GB). Don't try to read it all.
- Focus on patches/ directory for our modifications
- Use mach commands for building, not raw make
- Patches should be atomic and well-documented
- Test builds on Windows, Linux, and macOS
