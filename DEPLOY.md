# DEPLOYMENT & BUILD GUIDE

This document serves as the highly detailed deployment and environment setup guide for Bobzilla, migrating and expanding upon the foundational `docs/BUILD.md`.

## Prerequisites

### All Platforms
- **Python:** 3.8 or newer.
- **Rust:** Latest stable version (`rustup update stable`).
- **Git & Tools:** `git`, `git-cinnabar` (crucial for fetching the Mozilla source efficiently).
- **Hardware:** Minimum ~50GB free disk space; 16GB+ RAM (32GB strongly recommended for parallel builds).

### Windows
- Visual Studio 2022 with the "Desktop development with C++" workload installed.
- Windows 10/11 SDK.
- MozillaBuild package (must be installed and configured).

### Linux (Debian/Ubuntu)
```bash
sudo apt update
sudo apt install build-essential libgtk-3-dev libdbus-glib-1-dev \
    libpulse-dev libasound2-dev libx11-xcb-dev libxcb1-dev \
    libxcb-util0-dev nasm yasm clang lld
```

### macOS
- Xcode Command Line Tools (`xcode-select --install`).
- Homebrew packages: `brew install yasm nasm`.

## Bootstrap & Build Workflow

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/robertpelloni/bobzilla.git
   cd bobzilla
   ```

2. **Fetch Submodules & Source:**
   *Note: This step downloads the massive `mozilla-unified` codebase and will take time.*
   ```bash
   ./scripts/fetch-source.sh
   ```

3. **Apply Bobzilla Patches:**
   This applies our privacy, DRM, performance, UI, and branding modifications.
   ```bash
   ./scripts/apply-patches.sh
   ```

4. **Bootstrap Environment (First-Time Only):**
   ```bash
   ./scripts/build.sh bootstrap
   ```

5. **Build the Browser:**
   ```bash
   ./scripts/build.sh
   ```

6. **Run Locally:**
   ```bash
   cd mozilla-unified && ./mach run
   ```

## Build Options & Optimization

Edit `mozconfig.bobzilla` before building to apply custom optimizations:

- **Enable LTO (Link-Time Optimization):** Slower build, faster browser runtime.
  ```bash
  ac_add_options --enable-lto=cross
  ```
- **Enable PGO (Profile-Guided Optimization):** Requires a two-pass build.
  ```bash
  ac_add_options --enable-profile-generate  # Run first build
  ac_add_options --enable-profile-use       # Run second build after profiling
  ```
- **Parallel Compilation:**
  ```bash
  mk_add_options MOZ_MAKE_FLAGS="-j8"  # Adjust '8' to your CPU core count
  ```

## Packaging for Distribution

After a successful build, you can create distributable packages:

- **Windows Installer:**
  ```bash
  cd mozilla-unified
  ./mach package
  # Output: obj-bobzilla/dist/bobzilla-*.win64.installer.exe
  ```
- **Linux:**
  ```bash
  ./mach package
  # Output: obj-bobzilla/dist/bobzilla-*.tar.bz2
  ```
- **macOS:**
  ```bash
  ./mach package
  # Output: obj-bobzilla/dist/bobzilla-*.dmg
  ```

## Troubleshooting & Maintenance

- **Out of Memory during build:** Reduce parallel jobs (`MOZ_MAKE_FLAGS="-j4"`) or add swap space.
- **Rust Errors:** Ensure toolchains are up-to-date (`rustup update stable`).
- **Build fails after upstream update:**
  ```bash
  cd mozilla-unified
  ./mach clobber
  cd ..
  ./scripts/build.sh
  ```
