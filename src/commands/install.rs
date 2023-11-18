use crate::engine::InstallArgs;

pub fn download_pkgs(install_pkgs: InstallArgs) {
    let pkgname = install_pkgs.package;
    println!("{:?}", pkgname);
}
