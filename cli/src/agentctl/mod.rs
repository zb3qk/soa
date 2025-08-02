pub mod config;
pub mod env;

pub mod describe;
pub mod deploy;
pub mod invoke;
pub mod list;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum AgentCtlCommands {
    /// Manage configuration profiles
    #[command(subcommand)]
    Config(config::ConfigCommands),
    /// Manage environment profiles
    #[command(subcommand)]
    Env(env::EnvCommands),
    /// Describe an entity by id
    Describe(describe::DescribeArgs),
    /// Deploy an entity by id
    Deploy(deploy::DeployArgs),
    /// Invoke an entity with some input
    Invoke(invoke::InvokeArgs),
    /// List entities by id
    List(list::ListArgs),
}
