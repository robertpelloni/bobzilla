// =============================================================================
// BOBZILLA user.js - Arkenfox/LibreWolf Inspired Privacy Layer
// =============================================================================

// --- PREVENT WEBRTC LEAKS ---
// Force WebRTC to only use the default route (prevents VPN IP leaks)
user_pref("media.peerconnection.ice.default_address_only", true);
// Disable WebRTC completely if not needed (optional, uncomment to apply)
// user_pref("media.peerconnection.enabled", false);

// --- STRICT FINGERPRINTING RESISTANCE ---
user_pref("privacy.resistFingerprinting", true);
// Enable letterboxing to standardize window sizes
user_pref("privacy.resistFingerprinting.letterboxing", true);
// Ask for permission to read canvas data
user_pref("privacy.canvas.read.ask", true);

// --- DISABLE PREFETCHING / DNS LEAKS ---
user_pref("network.dns.disablePrefetch", true);
user_pref("network.dns.disablePrefetchFromHTTPS", true);
user_pref("network.predictor.enabled", false);
user_pref("network.prefetch-next", false);
user_pref("network.http.speculative-parallel-limit", 0);

// --- MANIFEST V2 EXTENSION SUPPORT ---
// Ensure legacy/MV2 extension APIs remain enabled for advanced ad-blocking
user_pref("extensions.manifestV3.enabled", false);
user_pref("extensions.legacy.enabled", true);

// --- DISABLE SAFE BROWSING (Optional Privacy Tradeoff) ---
// Prevents sending hashes of downloaded files/URLs to Google
user_pref("browser.safebrowsing.malware.enabled", false);
user_pref("browser.safebrowsing.phishing.enabled", false);
user_pref("browser.safebrowsing.downloads.enabled", false);

// --- DISABLE PASSWORD MANAGER ---
// Rely on external secure managers (Bitwarden, KeePass, etc.)
user_pref("signon.rememberSignons", false);

// --- CLEAR DATA ON SHUTDOWN ---
user_pref("privacy.sanitize.sanitizeOnShutdown", true);
user_pref("privacy.clearOnShutdown.cache", true);
user_pref("privacy.clearOnShutdown.cookies", true);
user_pref("privacy.clearOnShutdown.downloads", true);
user_pref("privacy.clearOnShutdown.formdata", true);
user_pref("privacy.clearOnShutdown.history", true);
user_pref("privacy.clearOnShutdown.offlineApps", true);
user_pref("privacy.clearOnShutdown.sessions", true);
user_pref("privacy.clearOnShutdown.siteSettings", false); // Keep site permissions

// --- HTTPS-ONLY ENFORCEMENT ---
// Force HTTPS-only across the browser (redundant catch-all against bobzilla.cfg)
user_pref("dom.security.https_only_mode", true);
user_pref("dom.security.https_only_mode_ever_enabled", true);
user_pref("dom.security.https_only_mode_send_http_background_request", false);
