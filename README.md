# TNTechHPConnect

Rust CLI that mirrors Google-Colab-style workflows for the **TN Tech University HPC clusters** (Impulse / Warp 1 + GENESIS/ASCEND):

- submit compute jobs (`sbatch` over SSH)
- check job queue status (`squeue`)
- check current job progress (`squeue` / `scontrol` / `sacct`)
- check allocated resources (`sinfo` / `sshare` / quota)
- fetch results back (`scp -r`)

Simple + fast: thin wrapper over the OpenSSH client you already have. No daemon, no heavy deps.

## Quick start

```bash
tntech-hpc config init
# edit ~/.config/tntech-hpc/config.toml -> ssh_target = "youruser@hpc.tntech.edu"
tntech-hpc config show

tntech-hpc submit --script examples/hello.sbatch --dry-run
tntech-hpc queue --dry-run
tntech-hpc status 123456 --dry-run
tntech-hpc resources --dry-run
tntech-hpc results 123456 --out ./results --dry-run
```

Drop `--dry-run` (or set `--ssh user@host`) to actually execute.

## Commands

| Command | Cluster equivalent |
|---|---|
| `submit --script job.sbatch` | `scp` + `sbatch` |
| `queue [--partition P]` | `squeue` |
| `status <jobid>` | `squeue -j` + `scontrol show job` + `sacct -j` |
| `resources` | `sinfo` + `sshare` + `df/quota` |
| `results <jobid>` | `scp -r` |
| `config init/show` | local `~/.config/tntech-hpc/config.toml` |
| `docs [--list]` | local `docs/` mirror |

## Docs mirror

`docs/` holds a local copy of all *publicly reachable* TN Tech HPC documentation
(fetched 2026-09-16). The canonical how-to KB lives behind TN Tech SSO at
`services.tntech.edu/TDClient/...` and cannot be scraped anonymously — see
`docs/GATED_DOCS_INDEX.md` for the article-ID index so an authenticated user can
export them.

## Private repo

This repo is intended to live as a **private** GitHub repo (`gh repo create --private`).
