use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "tntech-hpc",
    version,
    about = "Colab-like CLI for TN Tech HPC (Impulse / Warp 1): submit jobs, check queue, resources, progress, fetch results"
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOpts,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args, Debug, Clone)]
pub struct GlobalOpts {
    /// SSH destination, e.g. user@hpc.tntech.edu (overrides config file / env TNT_HPC_SSH)
    #[arg(long, global = true, env = "TNT_HPC_SSH")]
    pub ssh: Option<String>,

    /// Print the remote command instead of executing it
    #[arg(long, global = true, default_value_t = false)]
    pub dry_run: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Submit a batch script (wraps `sbatch` on the cluster)
    Submit(SubmitArgs),
    /// Show progress / state for one job (wraps `squeue` + `sacct`/`scontrol`)
    Status(StatusArgs),
    /// Show job queue status (wraps `squeue`)
    Queue(QueueArgs),
    /// Show allocated resources: partitions, nodes, QOS, storage (wraps `sinfo`, `sshare`, `df`)
    Resources(ResourcesArgs),
    /// Fetch result files back from the cluster (wraps `scp`)
    Results(ResultsArgs),
    /// Manage local connection config (~/.config/tntech-hpc/config.toml)
    Config(ConfigArgs),
    /// Interactive cluster login (stores identity + keychain password locally)
    Auth(AuthArgs),
    /// Show local copy of TN Tech HPC documentation
    Docs(DocsArgs),
}

#[derive(Args, Debug, Clone)]
pub struct SubmitArgs {
    /// Local sbatch script to upload + submit
    #[arg(long)]
    pub script: String,
    /// Remote working directory (default: $HOME)
    #[arg(long)]
    pub workdir: Option<String>,
    /// Extra sbatch args, e.g. --extra-args "--partition=gpu --gres=gpu:1"
    #[arg(long)]
    pub extra_args: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct StatusArgs {
    /// Slurm job id
    pub job_id: String,
}

#[derive(Args, Debug, Clone)]
pub struct QueueArgs {
    /// Show only your jobs (squeue -u $USER)
    #[arg(long, default_value_t = true)]
    pub mine: bool,
    /// Partition filter
    #[arg(long)]
    pub partition: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct ResourcesArgs {
    /// Include storage quota output
    #[arg(long, default_value_t = true)]
    pub storage: bool,
}

#[derive(Args, Debug, Clone)]
pub struct ResultsArgs {
    /// Slurm job id (used to suggest default remote path)
    pub job_id: String,
    /// Remote path to fetch (default: ~/tntech-hpc/<job-id>/)
    #[arg(long)]
    pub remote_path: Option<String>,
    /// Local destination directory
    #[arg(long, default_value = "./results")]
    pub out: String,
}

#[derive(Args, Debug, Clone)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigAction {
    /// Write default config file
    Init,
    /// Print current config
    Show,
}

#[derive(Args, Debug, Clone)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub action: AuthAction,
}

#[derive(Subcommand, Debug, Clone)]
pub enum AuthAction {
    /// Interactive login: prompt for host/user/creds, verify, store locally
    Login(LoginArgs),
    /// Remove the stored keychain password
    Logout,
    /// Show auth state (never prints secrets)
    Status,
}

#[derive(Args, Debug, Clone)]
pub struct LoginArgs {
    /// Prefill hostname (still prompts, editable)
    #[arg(long)]
    pub host: Option<String>,
    /// Prefill username (still prompts, editable)
    #[arg(long)]
    pub user: Option<String>,
    /// Skip the live `ssh` connection test
    #[arg(long, default_value_t = false)]
    pub no_test: bool,
}

#[derive(Args, Debug, Clone)]
pub struct DocsArgs {
    /// List bundled docs instead of showing index
    #[arg(long, default_value_t = false)]
    pub list: bool,
}
