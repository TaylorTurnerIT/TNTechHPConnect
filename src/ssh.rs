use anyhow::{Context, Result};
use std::process::Command;
use std::sync::OnceLock;

use crate::{config, secret};

/// Run `ssh <target> <remote_cmd>` (or print it with --dry-run).
/// Keeps operation simple + fast: shells out to the user's ssh/scp.
///
/// Auth resolution (no extra flags needed on callers):
/// - `-i <key>` is added automatically when `auth login` stored a key path.
/// - A keychain password (stored by `auth login`) is supplied non-interactively
///   via `sshpass -e` (password travels in env `SSHPASS`, never argv) when
///   `sshpass` is installed; otherwise ssh falls back to its own prompt.
pub fn ssh_run(target: &str, remote_cmd: &str, dry_run: bool) -> Result<()> {
    let pw = secret::get_password(keychain_account(target));
    if dry_run {
        match &pw {
            Some(_) if sshpass_available() => {
                println!("ssh {target} {remote_cmd}  # password from OS keychain via sshpass");
            }
            Some(_) => {
                println!("ssh {target} {remote_cmd}  # password stored but `sshpass` missing; ssh will prompt");
            }
            None => println!("ssh {target} {remote_cmd}"),
        }
        return Ok(());
    }
    let mut cmd = match pw {
        Some(p) if sshpass_available() => {
            let mut c = Command::new("sshpass");
            c.arg("-e").env("SSHPASS", p).arg("ssh");
            c
        }
        Some(_) => {
            eprintln!(
                "note: a password is stored but `sshpass` is not installed; falling back to interactive ssh prompt"
            );
            Command::new("ssh")
        }
        None => Command::new("ssh"),
    };
    if let Some(k) = configured_key() {
        cmd.arg("-i").arg(k);
    }
    let status = cmd
        .arg(target)
        .arg(remote_cmd)
        .status()
        .context("spawning ssh (is openssh installed?)")?;
    if !status.success() {
        anyhow::bail!("ssh exited with {status}");
    }
    Ok(())
}

/// Run `scp [extra...] <src> <dst>` with the same auth resolution as [`ssh_run`].
/// `target` is the `user@host` the transfer authenticates against
/// (it is usually embedded in src/dst already; it is only used for auth lookup).
pub fn scp_run(src: &str, dst: &str, extra: &[&str], target: &str, dry_run: bool) -> Result<()> {
    let pw = secret::get_password(keychain_account(target));
    if dry_run {
        let ex = extra.join(" ");
        match &pw {
            Some(_) if sshpass_available() => {
                println!("scp {ex} {src} {dst}  # password from OS keychain via sshpass");
            }
            Some(_) => {
                println!("scp {ex} {src} {dst}  # password stored but `sshpass` missing; scp will prompt");
            }
            None => println!("scp {ex} {src} {dst}"),
        }
        return Ok(());
    }
    let mut cmd = match pw {
        Some(p) if sshpass_available() => {
            let mut c = Command::new("sshpass");
            c.arg("-e").env("SSHPASS", p).arg("scp");
            c
        }
        Some(_) => {
            eprintln!(
                "note: a password is stored but `sshpass` is not installed; falling back to interactive scp prompt"
            );
            Command::new("scp")
        }
        None => Command::new("scp"),
    };
    if let Some(k) = configured_key() {
        cmd.arg("-i").arg(k);
    }
    let status = cmd
        .args(extra)
        .arg(src)
        .arg(dst)
        .status()
        .context("spawning scp (is openssh installed?)")?;
    if !status.success() {
        anyhow::bail!("scp exited with {status}");
    }
    Ok(())
}

/// Shell-escape a single argument for sh -c on the remote side.
pub fn sh_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Keychain lookup key: user part of `user@host`, else the whole target.
fn keychain_account(target: &str) -> &str {
    match target.split_once('@') {
        Some((user, _)) if !user.is_empty() => user,
        _ => target,
    }
}

/// Key path from local config (best effort; `None` when never logged in).
fn configured_key() -> Option<String> {
    let cfg = config::load().ok()?;
    let k = cfg.key_path?;
    if k.trim().is_empty() {
        None
    } else {
        Some(k)
    }
}

pub fn sshpass_available() -> bool {
    static AVAIL: OnceLock<bool> = OnceLock::new();
    *AVAIL.get_or_init(|| {
        Command::new("sshpass")
            .arg("-V")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    })
}
