mod cli;
mod commands;
mod config;
mod secret;
mod ssh;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Submit(args) => commands::submit::run(args, &cli.global),
        Command::Status(args) => commands::status::run(args, &cli.global),
        Command::Queue(args) => commands::queue::run(args, &cli.global),
        Command::Resources(args) => commands::resources::run(args, &cli.global),
        Command::Results(args) => commands::results::run(args, &cli.global),
        Command::Config(args) => commands::config_cmd::run(args),
        Command::Auth(args) => commands::auth::run(args),
        Command::Docs(args) => commands::docs::run(args),
    }
}
