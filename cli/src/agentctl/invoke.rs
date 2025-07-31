use crate::commands::OutputFormat;
use clap::Args;

#[derive(Args, Debug)]
pub struct InvokeArgs {
    #[arg(help = "ID of the entity")]
    pub id: String,
    #[arg(short, long, help = "Input to provide")]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
}

pub fn invoke(args: &InvokeArgs) {
    println!("Invoking {} with input {}", args.id, args.input);
    if let Some(fmt) = &args.output {
        println!("Output format: {:?}", fmt);
    }
}
