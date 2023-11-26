use crate::commands;
use crate::engine::args::CommandChoice;
use crate::log::abort;
use clap::Parser;

pub async fn start() {
    let cli = crate::engine::args::Cli::parse();

    let result = match cli.command {
        CommandChoice::Install(pkg_install_args) => {
            commands::install::install_packages(pkg_install_args).await
        }
        CommandChoice::Remove(pkg_uninstall_args) => {
            commands::remove::remove_pkg(pkg_uninstall_args).await
        }

        CommandChoice::List => commands::list::list_pkgs().await,
        CommandChoice::Search(pkg_search_args) => {
            commands::search::search_pkg(pkg_search_args).await
        }
    };

    if let Err(err) = result {
        abort(&format!("{}", err));
    }
}
