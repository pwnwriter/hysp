use crate::engine::hysp_ui::show_splashes;
use clap::{Args, Parser, Subcommand, ValueEnum};

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
    /// Install packages
    #[command(arg_required_else_help = true)]
    #[clap(name = "install", visible_alias = "i")]
    Install(InstallArgs),

    /// Uninstall packages
    #[command(arg_required_else_help = true)]
    #[clap(name = "remove", visible_alias = "r")]
    Remove(RemoveArgs),

    /// Search for a package
    #[command(arg_required_else_help = true)]
    #[clap(name = "search", visible_alias = "s")]
    Search(SearchArgs),

    /// List installed pkgs
    #[clap(name = "list", visible_alias = "l")]
    List(ListArgs),

    /// Check configuration health
    #[clap(name = "health", visible_alias = "h")]
    Health,

    /// Misc queries and helpers
    #[clap(name = "query", visible_alias = "q")]
    Query(QueryArgs),
}

#[derive(Args, Clone)]
pub struct InstallArgs {
    /// Name of the package to install
    #[arg(short,long,num_args(0..=100))] // Max 100 pkgs can be installed at once
    #[arg(required = true)]
    pub packages: Vec<String>,

    /// Force install a package
    #[arg(long)]
    #[arg(required = false)]
    pub force: bool,

    /// strip down console output
    #[arg(long, required = false)]
    pub quiet: bool,
}

#[derive(Args, Clone)]
pub struct RemoveArgs {
    /// Name of the package to Uninstall
    #[arg(short,long,num_args(0..=100))]
    #[arg(required = true)]
    pub packages: Vec<String>,

    /// Force remove a package
    #[arg(long, required = false)]
    pub force: bool,

    /// strip down console input/output
    #[arg(long, required = false)]
    pub quiet: bool,
}

#[derive(Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct SearchArgs {
    /// Name of the package to search
    #[arg(short, long)]
    #[arg(required = true)]
    pub package: String,

    /// Search for similar packages
    #[arg(long, value_enum, default_value = "raw")]
    pub mode: Modes,

    /// Define no. of max packages to show for fuzzy search
    #[arg(long, default_value = "4")]
    pub limit: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[allow(non_camel_case_types)]
pub enum Modes {
    /// Uses raw method to get package info
    raw,

    /// Uses metadata to get package info
    database,

    /// Grabs package with pkgname in name & description
    fuzzy, // TODO: set limit arg only for fuzzy mode
}

#[derive(Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct ListArgs {
    /// Print all available packages
    #[arg(long, required = false, conflicts_with = "installed")]
    pub available: bool,

    /// Print all installed packages
    #[arg(long, required = false)]
    pub installed: bool,

    /// Verbose print the package information
    #[arg(long, required = false)]
    pub verbose: bool,
}

#[derive(Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct QueryArgs {
    /// Print default hysp config
    #[arg(long, required = false)]
    pub print_default_config: bool,

    /// Print example package toml
    #[arg(long, required = false)]
    pub print_example_package: bool,
}
