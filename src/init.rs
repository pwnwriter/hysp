use clap::Parser;

pub async fn start() {
    let cli = crate::engine::args::Cli::parse();

    let result = match cli.command {
        crate::engine::CommandChoice::Install(pkg_install_args) => {
            crate::commands::install::download_pkgs(pkg_install_args).await
        }
        crate::engine::CommandChoice::Remove(pkg_uninstall_args) => {
            crate::commands::uninstall::remove_pkgs(pkg_uninstall_args).await
        }
        crate::engine::CommandChoice::Search(pkg_search_args) => {
            crate::commands::search::fetch_available(pkg_search_args).await
        }
        crate::engine::CommandChoice::List => crate::commands::list::print_installed_pkgs()
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
