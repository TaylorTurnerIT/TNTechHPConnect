use anyhow::Result;
use crate::cli::{ConfigAction, ConfigArgs};
use crate::config;

pub fn run(args: ConfigArgs) -> Result<()> {
    match args.action {
        ConfigAction::Init => {
            let p = config::init()?;
            println!("wrote {}", p.display());
        }
        ConfigAction::Show => {
            let cfg = config::load()?;
            println!("{}", toml::to_string_pretty(&cfg)?);
            if let Some(p) = config::config_path() {
                eprintln!("source: {}", p.display());
            }
        }
    }
    Ok(())
}
