use crate::engine::show_splashes;
use clap::{Args, Parser, Subcommand};

/// The HYSP CLI.
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

    /// List installed pkgs
    #[clap(name = "list")]
    List,

    /// Search a package
    #[command(arg_required_else_help = true)]
    #[clap(name = "search")]
    Search(SearchArgs),
}

#[derive(Args, Clone)]
pub struct InstallArgs {
    /// Name of the package to install
    #[arg(short, long)]
    #[arg(required = true)]
    pub package: String,

    /// Strip down the console output
    #[arg(long)]
    #[arg(required = false)]
    pub silent: bool,
}

#[derive(Args, Clone)]
pub struct RemoveArgs {
    /// Name of the package to Uninstall
    #[arg(short, long)]
    #[arg(required = true)]
    pub package: String,

    /// Strip down the console output
    #[arg(long)]
    #[arg(required = false)]
    pub silent: bool,
}

#[derive(Args, Clone)]
pub struct SearchArgs {
    /// Name of the package to search
    #[arg(short, long)]
    #[arg(required = true)]
    pub package: String,

    /// Strip down the console output
    #[arg(long)]
    #[arg(required = false)]
    pub silent: bool,
}
