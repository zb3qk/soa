use clap::Args;

#[derive(Args, Debug)]
pub struct StatusCommandArgs {
    #[arg(help = "ID of the agent")]
    pub id: String,
}
