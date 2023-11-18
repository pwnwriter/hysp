use crate::engine::args::Cli;
use clap::Parser;

mod commands;
mod engine;
mod log;

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        engine::CommandChoice::Install(pkg_install_args) => {
            commands::install::download_pkgs(pkg_install_args)
        }

        engine::CommandChoice::Remove(pkg_uninstall_args) => {
            commands::uninstall::remove_pkgs(pkg_uninstall_args)
        }
    };
}
