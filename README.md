# TNTechHPConnect

Rust CLI that mirrors Google-Colab-style workflows for the TN Tech University HPC:

- submit compute jobs (`sbatch` over SSH)
- check job queue status (`squeue`)
- check current job progress (`squeue` / `scontrol` / `sacct`)
- check allocated resources (`sinfo` / `sshare` / quota)
- fetch results back (`scp -r`)

Simple + fast: thin wrapper over OpenSSH client.

## Quick start

```bash
tntech-hpc auth login            # interactive: host, username, key or password; verifies + stores locally
tntech-hpc auth status           # show auth state (never prints secrets)

tntech-hpc submit --script examples/hello.sbatch --dry-run
tntech-hpc queue --dry-run
tntech-hpc status 123456 --dry-run
tntech-hpc resources --dry-run
tntech-hpc results 123456 --out ./results --dry-run
```

Drop `--dry-run` (or set `--ssh user@host`) to actually execute.

## Credentials

- Identity (host, username, key path) lives in `~/.config/tntech-hpc/config.toml`
  with `0600` permissions — editable by hand or via `config init/show`.
- Passwords are **never** written to disk: `auth login` puts them in the OS
  keychain (service `tntech-hpc`, account = username) via the `keyring` crate.
  On Linux this uses the D-Bus Secret Service (KWallet / GNOME Keyring), so
  build inside the provided devenv (`devenv shell`, supplies dbus headers).
- Non-interactive password use needs `sshpass` installed
  (e.g. `nix-shell -p sshpass`); without it, ssh/scp fall back to prompting.
  Key auth is recommended: login offers to generate an ed25519 key, prints the
  public key to register on the cluster, and tests the connection.
- `auth logout` removes the keychain entry. On headless machines without a
  dbus secret-service, login warns and proceeds without storing.

## Commands

| Command | Cluster equivalent |
|---|---|
| `submit --script job.sbatch` | `scp` + `sbatch` |
| `queue [--partition P]` | `squeue` |
| `status <jobid>` | `squeue -j` + `scontrol show job` + `sacct -j` |
| `resources` | `sinfo` + `sshare` + `df/quota` |
| `results <jobid>` | `scp -r` |
| `config init/show` | local `~/.config/tntech-hpc/config.toml` |
| `auth login/logout/status` | interactive login; OS keychain password storage |
| `docs [--list]` | local `docs/` mirror |

## Docs mirror

`docs/` holds a local copy of all publicly reachable TN Tech HPC documentation
(fetched 2026-09-16). The canonical how-to KB lives behind TN Tech SSO at
`services.tntech.edu/TDClient/...` and cannot be scraped anonymously — see
`docs/GATED_DOCS_INDEX.md` for the article-ID index so an authenticated user can
export them.
