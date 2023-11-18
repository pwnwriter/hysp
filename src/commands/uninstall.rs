use crate::engine::RemoveArgs;

pub fn remove_pkgs(uninstall_pkgs: RemoveArgs) {
    let pkgname = uninstall_pkgs.package;
    println!("{:?}", pkgname);
}
