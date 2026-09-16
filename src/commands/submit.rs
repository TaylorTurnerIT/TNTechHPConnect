use anyhow::Result;
use std::process::Command;

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

    // 1. mkdir + upload
    let mkdir_cmd = format!("ssh {} {:?}", target, format!("mkdir -p {workdir}/tntech-hpc"));
    let scp_cmd = format!("scp {} {target}:{remote_script}", ssh::sh_escape(&args.script));
    if global.dry_run {
        println!("{mkdir_cmd}");
        println!("{scp_cmd}");
    } else {
        let st = Command::new("ssh")
            .arg(&target)
            .arg(format!("mkdir -p {workdir}/tntech-hpc"))
            .status()?;
        anyhow::ensure!(st.success(), "remote mkdir failed");
        let st = Command::new("scp")
            .arg(&args.script)
            .arg(format!("{target}:{remote_script}"))
            .status()?;
        anyhow::ensure!(st.success(), "scp upload failed");
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
