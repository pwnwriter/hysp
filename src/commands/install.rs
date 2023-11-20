use crate::engine::data_parser::fetch_package_info;
use crate::engine::InstallArgs;

pub async fn download_pkgs(install_pkgs: InstallArgs) {
    let pkgname = install_pkgs.package;


    if install_pkgs.verbose {
        if let Ok(toml_info) = fetch_package_info(&pkgname).await {
            println!("{:?}", toml_info);
        } else {
            println!("Failed to fetch package information");
        }
    }
}
