use crate::engine::package_information::PackageInfo;
use crate::engine::{data_parser::fetch_and_process_toml, InstallArgs};
use crate::log::abort;

pub async fn download_pkgs(install_pkgs: InstallArgs) {
    let pkgname = install_pkgs.package;
    if let Err(err) = fetch_and_process_toml(&pkgname).await {
        abort(&format!("{}: {}", err, pkgname));
    }
}
