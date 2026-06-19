# ROADMAP

## Long-Term Structural Milestones

### 1. Architectural & Performance Perspectives
- **The "V8-Spider" Hybrid Engine:** Implement a Smart Engine Dispatcher. Use V8 for high-compute web apps (e.g., Google Sheets/Docs) while falling back to a Hardened SpiderMonkey for untrusted or privacy-sensitive sites.
- **WASM-Sandboxed Rendering:** Port critical parts of the Gecko engine to Rust and compile them to WASM. Sandbox the rendering of individual tabs into high-performance, memory-safe containers, isolated from each other and the host OS.
- **Investigate V8 embedding (research):** Explore embedding paths for V8 alongside SpiderMonkey.
- **Memory usage improvements:** Maximize Firefox's memory efficiency advantage over Chromium.
- **Startup time optimization:** Refactor initialization routines.

### 2. Privacy & Anti-Surveillance Perspectives
- **The "Shadow" Fingerprint Generator:** Autonomously Generate "Ghost" Fingerprints. Modify exposed headers, screen resolution, and font lists periodically to prevent session linking.
- **AI-Powered Ad-Nuker (The Sentinel):** Integrate a Local SLM (Small Language Model) that visually scans the page as it renders to identify and remove ads, bypassing Ad-Blocker Blocker scripts.
- **Network-level blocking capabilities:** Implement DNS-level blocking integration.

### 3. Product & Ecosystem Perspectives
- **Embedded "Bobcoin" P2P Marketplace:** Integrate the Bobcoin Wallet and a P2P browser-based store natively. Enable users to buy/sell digital assets securely within the browser chrome.
- **Decentralized "Permanent Web" Mode:** Integrate IPFS and Arweave natively. Autonomously serve mirrors for censored or 404 links.

### 4. UX & Customization Perspectives
- **The "Temporal" Tab Manager:** Implement a Timeline-based Session Engine. Allow users to scrub back in time to review and instantly resume prior session states.
- **Voice-Native Navigation:** Pivot Always-Listening tech into Browser Commands (e.g., "Bobzilla, open my trading dashboard"). Build a truly autonomous Agent Shell.
- **Floorp-style customization:** Deep integration of extensive UI modifications.

### 5. DRM/Streaming Pragmatism
- **Widevine L1 investigation:** Document and advocate regarding Google's DRM gatekeeping. Work on any achievable paths or workarounds.
- **Hardware video decoding improvements:** Ensure VAAPI/NVDEC function flawlessly.

### 6. Distribution & Packaging
- Windows installer.
- Linux packages (AppImage, Flatpak, deb, rpm).
- macOS dmg.
- Auto-update system (independent of Mozilla's infrastructure).
