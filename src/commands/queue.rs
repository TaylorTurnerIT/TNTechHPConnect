use anyhow::Result;
use crate::cli::{GlobalOpts, QueueArgs};
use crate::{config, ssh};

/// Job queue status: squeue (+ optional partition filter).
pub fn run(args: QueueArgs, global: &GlobalOpts) -> Result<()> {
    let cfg = config::load()?;
    let target = global.ssh.clone().unwrap_or(cfg.ssh_target);
    let mut remote = String::from("squeue --me 2>/dev/null || squeue -u \"$USER\"");
    if !args.mine {
        remote = String::from("squeue");
    }
    if let Some(p) = args.partition {
        remote.push_str(&format!(" -p {}", ssh::sh_escape(&p)));
    }
    ssh::ssh_run(&target, &remote, global.dry_run)
}
