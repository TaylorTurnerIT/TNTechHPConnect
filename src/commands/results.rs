use anyhow::Result;
use crate::cli::{GlobalOpts, ResultsArgs};
use crate::{config, ssh};

/// Fetch results back: scp -r target:remote_path ./results
pub fn run(args: ResultsArgs, global: &GlobalOpts) -> Result<()> {
    let cfg = config::load()?;
    let target = global.ssh.clone().unwrap_or(cfg.ssh_target);
    let remote_path = args
        .remote_path
        .unwrap_or_else(|| format!("~/tntech-hpc/{}/", args.job_id));
    std::fs::create_dir_all(&args.out)?;
    let src = format!("{target}:{remote_path}");
    ssh::scp_run(&src, &args.out, &["-r"], &target, global.dry_run)?;
    if !global.dry_run {
        println!("fetched {} -> {}", src, args.out);
    }
    Ok(())
}
