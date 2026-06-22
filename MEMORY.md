# Memory Document: Ongoing Observations

## Current State
- The project is a Firefox fork using a patch-based approach. We do not modify Firefox source directly but maintain patches in the `patches/` directory.
- `mozilla-unified` submodule is large (~3GB) and requires explicit fetching to manipulate.
- Configuration is largely driven by `settings/bobzilla.cfg` and `policies/policies.json`.

## Agent Directives
- Follow the patch-based workflow carefully.
- Prioritize atomic, well-documented changes over direct hacks.
