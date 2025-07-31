use crate::commands::OutputFormat;
use clap::Args;

#[derive(Args, Debug)]
pub struct ShowArgs {
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
}

pub fn show(args: &ShowArgs) {
    println!("Showing environment profiles...");
    if let Some(fmt) = &args.output {
        println!("Output format: {:?}", fmt);
    }
}
