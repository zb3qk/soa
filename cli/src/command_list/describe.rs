use clap::Args;

#[derive(Args, Debug)]
pub struct SendCommandArgs {
    #[arg(help = "ID of the agent")]
    pub id: String,

    #[arg(help = "Action to send (e.g., restart, sync-config)")]
    pub action: String,
}
