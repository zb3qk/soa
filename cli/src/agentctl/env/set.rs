use crate::commands::OutputFormat;
use clap::Args;

#[derive(Args, Debug)]
pub struct SetArgs {
    #[arg(help = "Profile ID")]
    pub profile_id: String,
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
}

pub fn set(args: &SetArgs) {
    println!("Setting environment profile to: {}", args.profile_id);
    if let Some(fmt) = &args.output {
        println!("Output format: {:?}", fmt);
    }
}
