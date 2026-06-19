# TODO

## Short-Term Features & Immediate Tasks

### Privacy (Day 1)
- [ ] Ensure all telemetry is disabled by default.
- [ ] Verify no Mozilla accounts integration by default.
- [ ] Validate removal of Pocket.
- [ ] Confirm no sponsored content is rendered.
- [ ] Set "Enhanced tracking protection" to strict by default.
- [ ] Enable fingerprinting resistance.
- [ ] Ensure HTTPS-only mode is the default.

### Performance
- [ ] Apply immediate SpiderMonkey optimizations.
- [ ] Turn on all performance flags in mozconfig (LTO, PGO).

### DRM/Streaming
- [ ] Ensure Widevine L3 is enabled and working (for baseline Netflix support).
- [ ] Optimize the video pipeline for the best possible quality at L3.
- [ ] Ensure hardware video decoding (VAAPI/NVDEC) works properly.

### Ad Blocking
- [ ] Preserve Manifest V2 permanently.
- [ ] Verify full webRequest API support.
- [ ] Implement built-in uBlock Origin (optional).

### UI/UX
- [ ] Integrate Vertical tabs natively.
- [ ] Implement Workspaces/tab groups.
- [ ] Apply a clean, minimal default theme.

### Repository / Build Environment
- [ ] Add the Firefox source (`mozilla-unified`) as a submodule if not already present.
- [ ] Apply LibreWolf-style privacy patches.
- [ ] Apply custom branding patches.
