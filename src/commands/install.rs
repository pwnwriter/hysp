use crate::engine::{data_parser::fetch_and_process_toml, InstallArgs};

pub async fn download_pkgs(install_pkgs: InstallArgs) {
    let pkgname = install_pkgs.package;
    println!("{:?}", pkgname);

    if install_pkgs.verbose {
        fetch_and_process_toml("test").await;
    }
}
