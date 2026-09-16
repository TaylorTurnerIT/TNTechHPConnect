use anyhow::Result;
use crate::cli::{GlobalOpts, StatusArgs};
use crate::{config, ssh};

/// Current job progress: squeue + scontrol + sacct tail.
pub fn run(args: StatusArgs, global: &GlobalOpts) -> Result<()> {
    let cfg = config::load()?;
    let target = global.ssh.clone().unwrap_or(cfg.ssh_target);
    let j = ssh::sh_escape(&args.job_id);
    let remote = format!(
        "squeue -j {j} ; echo '--- scontrol ---' ; scontrol show job {j} ; echo '--- sacct ---' ; sacct -j {j} --format=JobID,State,Elapsed,MaxRSS,MaxVMSize -P"
    );
    ssh::ssh_run(&target, &remote, global.dry_run)
}
