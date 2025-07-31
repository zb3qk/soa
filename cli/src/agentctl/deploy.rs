use crate::commands::OutputFormat;
use clap::Args;

#[derive(Args, Debug)]
pub struct DeployArgs {
    #[arg(help = "ID of the entity")]
    pub id: String,
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
}

pub fn deploy(args: &DeployArgs) {
    println!("Deploying {}", args.id);
    if let Some(fmt) = &args.output {
        println!("Output format: {:?}", fmt);
    }
}
