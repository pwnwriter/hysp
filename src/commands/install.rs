use crate::engine::data_parser::fetch_package_info;
use crate::engine::InstallArgs;

pub async fn download_pkgs(install_pkgs: InstallArgs) -> Result<(), anyhow::Error> {
    let pkgname = &install_pkgs.package; 

    let toml_info = fetch_package_info(pkgname).await?; 

    if install_pkgs.verbose {
        dbg!("{:?}", toml_info);
    }

    Ok(()) 
}
