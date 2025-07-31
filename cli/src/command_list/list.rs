use clap::Args;

#[derive(Args, Debug)]
pub struct ListCommandArgs {
    #[arg(short, long)]
    pub filter: Option<String>,
}

// Macro is no longer needed, but if you want to keep a macro for future expansion:
// macro_rules! list_command_args {
//     () => { ListCommandArgs };
// }
