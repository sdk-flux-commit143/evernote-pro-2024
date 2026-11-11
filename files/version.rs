//! Static build info for Evernote Pro 2024.

pub const BUILD_VERSION: &str  = "v3.7.1";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "8611978632f9";
pub const BUILD_TIMESTAMP: u64 = 1785107523;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Evernote Pro 2024", BUILD_VERSION, BUILD_CHANNEL)
}
