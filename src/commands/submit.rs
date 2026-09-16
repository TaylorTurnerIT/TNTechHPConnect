use anyhow::Result;

use crate::cli::{GlobalOpts, SubmitArgs};
use crate::{config, ssh};

/// Upload script via scp then `sbatch` it — the Colab "run cell" equivalent.
pub fn run(args: SubmitArgs, global: &GlobalOpts) -> Result<()> {
    let cfg = config::load()?;
    let target = global.ssh.clone().unwrap_or(cfg.ssh_target);
    let workdir = args.workdir.unwrap_or(cfg.default_workdir);
    let extra = args.extra_args.unwrap_or_default();

    let script_name = std::path::Path::new(&args.script)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "job.sbatch".to_string());
    let remote_script = format!("{workdir}/tntech-hpc/{script_name}");

    // 1. mkdir + upload (auth-aware: keychain password applied via sshpass when present)
    let mkdir_remote = format!("mkdir -p {workdir}/tntech-hpc");
    if global.dry_run {
        ssh::ssh_run(&target, &mkdir_remote, true)?;
        ssh::scp_run(
            &args.script,
            &format!("{target}:{remote_script}"),
            &[],
            &target,
            true,
        )?;
    } else {
        ssh::ssh_run(&target, &mkdir_remote, false)?;
        ssh::scp_run(
            &args.script,
            &format!("{target}:{remote_script}"),
            &[],
            &target,
            false,
        )?;
    }

    // 2. sbatch
    let remote = format!(
        "cd {} && sbatch {} {}",
        workdir,
        extra,
        ssh::sh_escape(&remote_script)
    );
    ssh::ssh_run(&target, &remote, global.dry_run)
}
