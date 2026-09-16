use anyhow::{Context, Result};
use std::process::Command;

/// Run `ssh <target> <remote_cmd>` (or print it with --dry-run).
/// Keeps operation simple + fast: shells out to the user's ssh/scp.
pub fn ssh_run(target: &str, remote_cmd: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("ssh {target} {remote_cmd}");
        return Ok(());
    }
    let status = Command::new("ssh")
        .arg(target)
        .arg(remote_cmd)
        .status()
        .context("spawning ssh (is openssh installed?)")?;
    if !status.success() {
        anyhow::bail!("ssh exited with {status}");
    }
    Ok(())
}

/// Shell-escape a single argument for sh -c on the remote side.
pub fn sh_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
