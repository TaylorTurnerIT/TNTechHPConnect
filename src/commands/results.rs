use anyhow::Result;
use std::process::Command;
use crate::cli::{GlobalOpts, ResultsArgs};
use crate::config;

/// Fetch results back: scp -r target:remote_path ./results
pub fn run(args: ResultsArgs, global: &GlobalOpts) -> Result<()> {
    let cfg = config::load()?;
    let target = global.ssh.clone().unwrap_or(cfg.ssh_target);
    let remote_path = args
        .remote_path
        .unwrap_or_else(|| format!("~/tntech-hpc/{}/", args.job_id));
    std::fs::create_dir_all(&args.out)?;
    let src = format!("{target}:{remote_path}");
    if global.dry_run {
        println!("scp -r {src} {}", args.out);
        return Ok(());
    }
    let st = Command::new("scp").arg("-r").arg(&src).arg(&args.out).status()?;
    anyhow::ensure!(st.success(), "scp fetch failed");
    println!("fetched {} -> {}", src, args.out);
    Ok(())
}
