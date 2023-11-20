use crate::engine::RemoveArgs;

pub async fn remove_pkgs(uninstall_pkgs: RemoveArgs)-> Result<(), anyhow::Error> {
    let pkgname = uninstall_pkgs.package;
    println!("{:?}", pkgname);
    Ok(()) 
}
