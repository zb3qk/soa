use crate::commands::OutputFormat;
use clap::Args;

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(help = "ID of the entity")]
    pub id: String,
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
}

pub fn list(args: &ListArgs) {
    println!("Listing {}", args.id);
    if let Some(fmt) = &args.output {
        println!("Output format: {:?}", fmt);
    }
}
