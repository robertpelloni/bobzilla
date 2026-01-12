# bobzilla Build Instructions

## Prerequisites

### All Platforms
- Python 3.8+
- Rust (latest stable)
- Git
- git-cinnabar (for fetching Mozilla source)
- ~50GB free disk space
- 16GB+ RAM (32GB recommended for parallel builds)

### Windows
- Visual Studio 2022 with C++ workload
- Windows 10/11 SDK
- MozillaBuild package

### Linux (Debian/Ubuntu)
```bash
sudo apt install build-essential libgtk-3-dev libdbus-glib-1-dev \
    libpulse-dev libasound2-dev libx11-xcb-dev libxcb1-dev \
    libxcb-util0-dev nasm yasm clang lld
```

### macOS
- Xcode Command Line Tools
- Homebrew packages: `brew install yasm nasm`

## Quick Start

```bash
# Clone bobzilla
git clone https://github.com/robertpelloni/bobzilla.git
cd bobzilla

# Fetch Firefox source (this takes a while)
./scripts/fetch-source.sh

# Apply bobzilla patches
./scripts/apply-patches.sh

# Bootstrap build environment (first time only)
./scripts/build.sh bootstrap

# Build
./scripts/build.sh

# Run
cd mozilla-unified && ./mach run
```

## Build Options

### mozconfig Customization

Edit `mozconfig.bobzilla` before building:

```bash
# Enable LTO (slower build, faster browser)
ac_add_options --enable-lto=cross

# Enable PGO (requires two builds)
ac_add_options --enable-profile-generate  # First build
ac_add_options --enable-profile-use       # Second build after profiling

# Debug build
ac_add_options --enable-debug
ac_add_options --disable-optimize
```

### Parallel Build

Adjust parallel jobs in mozconfig:
```bash
mk_add_options MOZ_MAKE_FLAGS="-j8"  # Use 8 cores
```

## Packaging

### Windows Installer
```bash
cd mozilla-unified
./mach package
# Output: obj-bobzilla/dist/bobzilla-*.win64.installer.exe
```

### Linux
```bash
./mach package
# Output: obj-bobzilla/dist/bobzilla-*.tar.bz2
```

### macOS
```bash
./mach package
# Output: obj-bobzilla/dist/bobzilla-*.dmg
```

## Troubleshooting

### Out of Memory
- Close other applications
- Reduce parallel jobs: `MOZ_MAKE_FLAGS="-j4"`
- Add swap space

### Rust Errors
```bash
rustup update stable
rustup default stable
```

### Missing Dependencies (Linux)
```bash
./mach bootstrap
```

### Build Fails After Update
```bash
./mach clobber  # Clean build directory
./mach build
```

## CI/CD

GitHub Actions workflow coming soon for automated builds on:
- Windows x64
- Linux x64
- macOS x64 (Intel)
- macOS arm64 (Apple Silicon)
