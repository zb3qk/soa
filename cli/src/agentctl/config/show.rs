use crate::commands::OutputFormat;
use clap::Args;
use super::file::{Config, config_path};

#[derive(Args, Debug)]
pub struct ShowArgs {
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
}

pub fn show(args: &ShowArgs) {
    let cfg = Config::load();
    println!("Current configuration at {}:", config_path().display());
    if let Some(of) = cfg.output_format {
        println!("output_format = {}", of);
    } else {
        println!("No configuration found");
    }
    if let Some(fmt) = &args.output {
        println!("Output format: {:?}", fmt);
    }
}
