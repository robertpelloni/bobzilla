#!/bin/bash
# Fetch Firefox source for bobzilla
# Uses Mozilla's git-cinnabar for faster clones

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SOURCE_DIR="$ROOT_DIR/mozilla-unified"

# Firefox ESR version to track (update this for new ESR releases)
FIREFOX_VERSION="esr128"

echo "=== Fetching Firefox source ==="
echo "This will take a while (~3GB download, ~15GB extracted)"

if [ -d "$SOURCE_DIR" ]; then
    echo "Source directory exists. Updating..."
    cd "$SOURCE_DIR"
    git fetch origin
    git checkout $FIREFOX_VERSION
    git pull origin $FIREFOX_VERSION
else
    echo "Cloning Firefox source (using GitHub mirror for Git access)..."
    
    # Option 1: Use Mozilla's official Mercurial (slower but official)
    # pip install mercurial git-cinnabar
    # git clone hg::https://hg.mozilla.org/mozilla-unified "$SOURCE_DIR"
    
    # Option 2: Use a Git mirror (faster)
    # Note: Mozilla doesn't maintain an official Git mirror
    # We'll set up tracking of ESR releases
    
    mkdir -p "$SOURCE_DIR"
    cd "$SOURCE_DIR"
    git init
    
    echo "Fetching from Mozilla Mercurial via git-cinnabar..."
    echo "(Install git-cinnabar first: https://github.com/glandium/git-cinnabar)"
    
    # Add Mozilla remote
    git remote add mozilla hg::https://hg.mozilla.org/mozilla-unified || true
    
    echo ""
    echo "=== MANUAL STEPS REQUIRED ==="
    echo "1. Install git-cinnabar: https://github.com/glandium/git-cinnabar"
    echo "2. Run: git fetch mozilla"
    echo "3. Run: git checkout -b bobzilla mozilla/$FIREFOX_VERSION"
    echo ""
    echo "Alternatively, download source tarball from:"
    echo "https://archive.mozilla.org/pub/firefox/releases/"
fi

echo "=== Done ==="
