//! NRS build switches on the Rust side.
//!
//! The consumer sign-in providers (Sora, Midjourney, Grok and World Labs web
//! accounts) capture a browser login and replay it with a browser-like TLS
//! fingerprint (`browser_emulation`). That can breach those services' terms
//! and get the account closed, so their sign-in windows only open when the
//! app is started with NRS_ENABLE_CONSUMER_LOGINS=1. The code stays intact.

pub const CONSUMER_LOGINS_OFF_MESSAGE: &str =
  "Signing in to this service is switched off in the NRS Craft build. Ask Justin if you need it.";

pub fn consumer_logins_enabled() -> bool {
  std::env::var("NRS_ENABLE_CONSUMER_LOGINS").map(|v| v == "1").unwrap_or(false)
}

#[cfg(test)]
mod tests {
  use super::consumer_logins_enabled;

  #[test]
  fn off_unless_set_to_one() {
    // Not set in the test environment.
    if std::env::var("NRS_ENABLE_CONSUMER_LOGINS").is_err() {
      assert!(!consumer_logins_enabled());
    }
  }
}
