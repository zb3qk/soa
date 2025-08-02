use crate::commands::OutputFormat;
use clap::Args;
use super::file::{Config, config_path};

#[derive(Args, Debug)]
pub struct AddArgs {
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
}

pub fn add(args: &AddArgs) {
    let mut cfg = Config::load();
    println!("Adding configuration...");
    if let Some(fmt) = &args.output {
        println!("Output format: {:?}", fmt);
        cfg.output_format = Some(fmt.as_str().to_string());
    }
    cfg.save();
    println!("Configuration written to {}", config_path().display());
}
