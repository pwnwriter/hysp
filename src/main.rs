use crate::engine::args::Cli;
use clap::Parser;

mod commands;
mod engine;
mod log;

//  asynchronous entry point where the magic happens :dizzy: 
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        engine::CommandChoice::Install(pkg_install_args) => {
            commands::install::download_pkgs(pkg_install_args).await
        }

        engine::CommandChoice::Remove(pkg_uninstall_args) => {
            commands::uninstall::remove_pkgs(pkg_uninstall_args)
        }
    };
}
