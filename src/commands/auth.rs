use anyhow::{Context, Result};
use dialoguer::{Confirm, Input, Password, Select};
use std::process::Command;

use crate::cli::{AuthArgs, AuthAction, LoginArgs};
use crate::{config, secret, ssh};

pub fn run(args: AuthArgs) -> Result<()> {
    match args.action {
        AuthAction::Login(l) => run_login(l),
        AuthAction::Logout => run_logout(),
        AuthAction::Status => run_status(),
    }
}

/// Interactive login: prompt for host/user/creds, verify against the cluster,
/// store identity in config.toml (0600) + password in the OS keychain.
pub fn run_login(args: LoginArgs) -> Result<()> {
    let mut cfg = config::load().unwrap_or_default();

    let host_default = args
        .host
        .or(cfg.host.clone())
        .unwrap_or_else(|| "login.hpc.tntech.edu".to_string());
    let host: String = Input::new()
        .with_prompt("HPC hostname")
        .default(host_default)
        .interact_text()?;
    let host = host.trim().to_string();
    anyhow::ensure!(!host.is_empty(), "hostname cannot be empty");

    let user_default = args.user.or(cfg.username.clone()).unwrap_or_default();
    let username: String = if user_default.is_empty() {
        Input::new().with_prompt("Username").interact_text()?
    } else {
        Input::new()
            .with_prompt("Username")
            .default(user_default)
            .interact_text()?
    };
    let username = username.trim().to_string();
    anyhow::ensure!(!username.is_empty(), "username cannot be empty");
    let target = format!("{username}@{host}");

    let methods = ["SSH key (recommended)", "Password (stored in OS keychain)"];
    let choice = Select::new()
        .with_prompt("Authentication method")
        .items(&methods)
        .default(0)
        .interact()?;

    if choice == 0 {
        login_with_key(&mut cfg, &username, &host, &target, args.no_test)?;
    } else {
        login_with_password(&mut cfg, &username, &host, &target, args.no_test)?;
    }

    let path = config::save(&cfg)?;
    println!("saved identity to {}", path.display());
    Ok(())
}

fn login_with_key(
    cfg: &mut config::AppConfig,
    _username: &str,
    _host: &str,
    target: &str,
    no_test: bool,
) -> Result<()> {
    let home_ssh = dirs::home_dir()
        .map(|h| h.join(".ssh").join("id_ed25519"))
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "~/.ssh/id_ed25519".to_string());
    let default_key = cfg.key_path.clone().unwrap_or(home_ssh);
    let key: String = Input::new()
        .with_prompt("Private key path")
        .default(default_key)
        .interact_text()?;
    let key = shellexpand_tilde(&key);

    if !std::path::Path::new(&key).exists() {
        let gen = Confirm::new()
            .with_prompt(format!("{key} does not exist. Generate a new ed25519 key there?"))
            .default(true)
            .interact()?;
        if gen {
            if let Some(parent) = std::path::Path::new(&key).parent() {
                std::fs::create_dir_all(parent)?;
            }
            let st = Command::new("ssh-keygen")
                .args(["-t", "ed25519", "-f", &key, "-N", ""])
                .status()
                .context("spawning ssh-keygen")?;
            anyhow::ensure!(st.success(), "ssh-keygen failed");
        } else {
            anyhow::bail!("key file not found; re-run login after creating one");
        }
    }

    // Show the public key so the user can register it on the cluster/portal.
    let pubkey = format!("{key}.pub");
    if std::path::Path::new(&pubkey).exists() {
        let text = std::fs::read_to_string(&pubkey)?;
        println!("\nYour public key (add to ~/.ssh/authorized_keys on the cluster):\n{text}");
    }

    if !no_test {
        println!("testing key auth against {target} ...");
        let st = Command::new("ssh")
            .args([
                "-i",
                &key,
                "-o",
                "BatchMode=yes",
                "-o",
                "ConnectTimeout=15",
                target,
                "true",
            ])
            .status()
            .context("spawning ssh for connection test")?;
        if !st.success() {
            eprintln!("warning: key auth test failed ({st}).");
            eprintln!("If the key is new, append the public key above to ~/.ssh/authorized_keys on the cluster, then re-run `auth login`.");
            let keep = Confirm::new()
                .with_prompt("Save this identity anyway?")
                .default(false)
                .interact()?;
            if !keep {
                anyhow::bail!("login aborted; identity not saved");
            }
        } else {
            println!("connection test OK");
        }
    }

    let (username, host) = split_target(target);
    cfg.ssh_target = target.to_string();
    cfg.host = Some(host);
    cfg.username = Some(username);
    cfg.key_path = Some(key);
    Ok(())
}

fn login_with_password(
    cfg: &mut config::AppConfig,
    username: &str,
    host: &str,
    target: &str,
    no_test: bool,
) -> Result<()> {
    let pw1: String = Password::new()
        .with_prompt("HPC password")
        .allow_empty_password(false)
        .interact()?;
    let pw2: String = Password::new()
        .with_prompt("Confirm HPC password")
        .allow_empty_password(false)
        .interact()?;
    anyhow::ensure!(pw1 == pw2, "passwords do not match");

    match secret::set_password(username, &pw1) {
        Ok(()) => println!("password stored in OS keychain (service `tntech-hpc`)"),
        Err(e) => {
            eprintln!("warning: could not use OS keychain: {e:#}");
            eprintln!("On headless machines the secret-service/dbus keychain may be unavailable.");
            let cont = Confirm::new()
                .with_prompt("Continue without storing (ssh will prompt each time)?")
                .default(true)
                .interact()?;
            if !cont {
                anyhow::bail!("login aborted; nothing saved");
            }
        }
    }

    if !ssh::sshpass_available() {
        eprintln!("note: `sshpass` is not installed, so the stored password cannot be applied non-interactively;");
        eprintln!("install it (e.g. `nix-shell -p sshpass`) or rely on interactive ssh prompts.");
    }
    if cfg.key_path.as_deref().is_some() {
        println!("note: a key path is also configured; ssh will try the key first, then the stored password.");
    }

    if !no_test {
        println!("testing connection to {target} ...");
        ssh::ssh_run(target, "true", false)?;
        println!("connection test OK");
    }

    cfg.ssh_target = target.to_string();
    cfg.host = Some(host.to_string());
    cfg.username = Some(username.to_string());
    Ok(())
}

pub fn run_logout() -> Result<()> {
    let cfg = config::load().unwrap_or_default();
    let username = match cfg.username.clone() {
        Some(u) => u,
        None => Input::new().with_prompt("Username to forget").interact_text()?,
    };
    let username = username.trim().to_string();
    anyhow::ensure!(!username.is_empty(), "username cannot be empty");
    if secret::delete_password(&username)? {
        println!("removed stored password for `{username}` from OS keychain");
    } else {
        println!("no stored password for `{username}`");
    }
    println!("note: config identity ({}) left in place; re-run `auth login` to change it.",
        config::config_path()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "<unknown>".to_string()));
    Ok(())
}

pub fn run_status() -> Result<()> {
    let cfg = config::load().unwrap_or_default();
    println!("ssh target : {}", cfg.ssh_target);
    println!("host       : {}", cfg.host.as_deref().unwrap_or("-"));
    println!("username   : {}", cfg.username.as_deref().unwrap_or("-"));
    match cfg.key_path.as_deref() {
        Some(k) => {
            let exists = std::path::Path::new(&shellexpand_tilde(k)).exists();
            println!("key path   : {k} ({})", if exists { "exists" } else { "MISSING" });
        }
        None => println!("key path   : -"),
    }
    let stored = cfg
        .username
        .as_deref()
        .and_then(secret::get_password)
        .is_some();
    println!("keychain pw: {}", if stored { "stored" } else { "none" });
    println!("sshpass    : {}", if ssh::sshpass_available() { "installed" } else { "missing (needed for non-interactive password auth)" });
    Ok(())
}

fn split_target(target: &str) -> (String, String) {
    match target.split_once('@') {
        Some((u, h)) => (u.to_string(), h.to_string()),
        None => (target.to_string(), String::new()),
    }
}

fn shellexpand_tilde(s: &str) -> String {
    if let Some(rest) = s.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest).to_string_lossy().to_string();
        }
    }
    s.to_string()
}
