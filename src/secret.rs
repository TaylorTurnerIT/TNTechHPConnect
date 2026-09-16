//! OS-keychain storage for the cluster password.
//!
//! Secrets never touch `config.toml` (which holds only host/username/key path
//! with `0600` permissions). Lookup key is the cluster username.

use anyhow::{Context, Result};

const SERVICE: &str = "tntech-hpc";

pub fn set_password(account: &str, password: &str) -> Result<()> {
    let entry = keyring::Entry::new(SERVICE, account).context("opening OS keychain")?;
    entry
        .set_password(password)
        .context("storing password in OS keychain")
}

pub fn get_password(account: &str) -> Option<String> {
    keyring::Entry::new(SERVICE, account)
        .ok()?
        .get_password()
        .ok()
}

pub fn delete_password(account: &str) -> Result<bool> {
    let entry = match keyring::Entry::new(SERVICE, account) {
        Ok(e) => e,
        Err(_) => return Ok(false),
    };
    match entry.delete_credential() {
        Ok(()) => Ok(true),
        Err(keyring::Error::NoEntry) => Ok(false),
        Err(e) => Err(e).context("removing password from OS keychain"),
    }
}
