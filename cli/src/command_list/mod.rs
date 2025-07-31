pub mod list;
mod status;
mod describe;

use clap::{Subcommand, Args};

#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    List,
    Cancel(CancelTaskArgs),
    Assign(AssignTaskArgs),
}

#[derive(Args, Debug)]
pub struct CancelTaskArgs {
    #[arg(help = "ID of the task to cancel")]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct AssignTaskArgs {
    #[arg(help = "ID of the task")]
    pub task_id: String,

    #[arg(help = "Agent to assign to")]
    pub to: String,
}
