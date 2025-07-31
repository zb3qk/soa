pub mod add;
pub mod show;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Add a configuration profile
    Add(add::AddArgs),
    /// Show current configuration
    Show(show::ShowArgs),
}
