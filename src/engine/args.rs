use crate::engine::show_splashes;
use clap::{Args, Parser, Subcommand};

/// The SEREN CLI.
#[derive(Parser)]
#[command(author, version, about = show_splashes(), long_about = show_splashes())]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// The command to execute.
    #[clap(subcommand)]
    pub command: CommandChoice,
}

#[derive(Subcommand)]
pub enum CommandChoice {
    /// Install a package
    #[command(arg_required_else_help = true)]
    #[clap(name = "install")]
    Install(InstallArgs),

    /// Uninstall a package
    #[command(arg_required_else_help = true)]
    #[clap(name = "remove")]
    Remove(RemoveArgs),
}

#[derive(Args, Clone)]
pub struct InstallArgs {
    /// Name of the package to install
    #[arg(short, long)]
    #[arg(required = true)]
    pub package: String,

    #[arg(short, long)]
    #[arg(required = true)]
    pub verbose: bool,
}

#[derive(Args, Clone)]
pub struct RemoveArgs {
    /// Name of the package to Uninstall
    #[arg(short, long)]
    #[arg(required = true)]
    pub package: String,
}
