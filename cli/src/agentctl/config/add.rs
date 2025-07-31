use crate::commands::OutputFormat;
use clap::Args;

#[derive(Args, Debug)]
pub struct AddArgs {
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
}

pub fn add(args: &AddArgs) {
    println!("Adding configuration...");
    if let Some(fmt) = &args.output {
        println!("Output format: {:?}", fmt);
    }
}
