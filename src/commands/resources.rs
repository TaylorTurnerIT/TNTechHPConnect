use anyhow::Result;
use crate::cli::{GlobalOpts, ResourcesArgs};
use crate::{config, ssh};

/// Allocated resources: partitions/nodes + fairshare + storage.
pub fn run(args: ResourcesArgs, global: &GlobalOpts) -> Result<()> {
    let cfg = config::load()?;
    let target = global.ssh.clone().unwrap_or(cfg.ssh_target);
    let mut remote =
        String::from("sinfo -o '%P %.5a %.10l %.6D %.6t %N' ; echo '--- sshare ---' ; sshare -U 2>/dev/null || sshare -u \"$USER\"");
    if args.storage {
        remote.push_str(" ; echo '--- quota ---' ; df -h \"$HOME\" ; quota -s 2>/dev/null || true");
    }
    ssh::ssh_run(&target, &remote, global.dry_run)
}
