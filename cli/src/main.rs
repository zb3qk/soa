use clap::{Parser, Subcommand};
mod commands;
mod agentctl;
mod command_list;
use command_list::list::ListCommandArgs;
use command_list::status::StatusCommandArgs;
use command_list::describe::SendCommandArgs;
use crate::command_list::TaskCommands;

#[derive(Parser)]
#[command(name = "agentctl")]
#[command(about = "Control plane CLI for managing agents", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all registered agents
    List(ListCommandArgs),

    /// Get status for a specific agent
    Status(StatusCommandArgs),

    /// Send command to an agent
    Send(SendCommandArgs),

    /// View or manage tasks
    Tasks {
        #[command(subcommand)]
        subcommand: TaskCommands,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List(args) => commands::list_agents(args.filter),
        Commands::Status(args) => commands::status_agent(&args.id),
        Commands::Send(args) => commands::send_command(&args.id, &args.action),
        Commands::Tasks { subcommand } => match subcommand {
            TaskCommands::List => commands::list_tasks(),
            TaskCommands::Cancel(args) => commands::cancel_task(&args.id),
            TaskCommands::Assign(args) => commands::assign_task(&args.task_id, &args.to),
        },
    }
}
