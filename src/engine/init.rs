use crate::commands;
use crate::engine::args::CommandChoice;
use crate::engine::msgx::abort;
use clap::Parser;

pub async fn exec() {
    let cli = crate::engine::args::Cli::parse();

    let result = match cli.command {
        CommandChoice::Install(pkg_install_args) => {
            commands::install::install_packages(pkg_install_args).await
        }

        CommandChoice::Remove(pkg_uninstall_args) => {
            commands::remove::remove_pkgs(pkg_uninstall_args).await
        }

        CommandChoice::Search(pkg_search_args) => {
            commands::search::search_pkgs(pkg_search_args).await
        }

        CommandChoice::List(pkg_list_args) => commands::list::list_pkgs(pkg_list_args).await,
        CommandChoice::Health => commands::health::check_health().await,
    };

    if let Err(err) = result {
        abort(&format!("{}", err));
    }
}
