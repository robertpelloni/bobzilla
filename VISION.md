# Vision Document: bobzilla

## Ultimate Goal
**bobzilla** aims to be the browser that doesn't suck. It combines the privacy DNA of Firefox, the performance competitiveness of Chromium/V8, proper Widevine L1 DRM support for 4K streaming, zero surveillance out-of-the-box, and a sandboxed MicroVM virtualization engine for native legacy and Java application payloads.

## Core Philosophy
1. **Privacy by default:** No opt-out telemetry, sponsored stories, or hidden network pings. Disabled natively at the C++ and Gecko engine levels.
2. **Performance matters:** Focus on SpiderMonkey optimization, jemalloc aggressive memory reclamation, and demand-paged disk streaming via HTTP Range Requests.
3. **DRM pragmatism:** Widevine L1 hardware spoofing supported to ensure uncompromised 4K media consumption on streaming providers.
4. **Ad Blocking Superiority:** Manifest V2 preserved indefinitely for uBlock Origin superiority, augmented by native engine-level network blocking.
5. **Unified Mission Control:** Single consolidated settings dashboard directly in browser preferences (`about:preferences#privacy`) categorized by high-value capabilities.
6. **Isolated Payload Sandboxing:** The JavaSandbox architecture isolates web-delivered Java applications within stateless Firecracker MicroVM containers backed by FUSE streaming.
