# Deployment Instructions: bobzilla

## Build Environment
Requires Python 3.x, Rust, and Visual Studio 2022 (Windows) or GCC/Clang (Linux/macOS). Expect ~50GB of disk space.

## Steps
1. Fetch source: `./scripts/fetch-source.sh`
2. Bootstrap: `./mach bootstrap`
3. Configure: `cp mozconfig.bobzilla mozilla-unified/mozconfig`
4. Build: `./scripts/build.sh`
5. Run: `cd mozilla-unified && ./mach run`
