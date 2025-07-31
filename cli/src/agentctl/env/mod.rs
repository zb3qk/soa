pub mod show;
pub mod set;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum EnvCommands {
    /// Show available environment profiles
    Show(show::ShowArgs),
    /// Set an environment profile
    Set(set::SetArgs),
}
