use crate::engine::dirs;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Arc;

pub fn print_installed_pkgs() -> Result<(), anyhow::Error> {
    let installed_pkgs = get_installed_pkgs()?;
    for pkg in installed_pkgs.iter() {
        println!("{}", pkg);
    }
    Ok(())
}

fn get_installed_pkgs() -> Result<Arc<Vec<String>>, io::Error> {
    let bin_dir: &Path = dirs::HYSP_BIN_DIR.as_ref();
    let mut binaries = Vec::new();

    if let Ok(entries) = fs::read_dir(bin_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    binaries.push(file_name.to_string());
                }
            }
        }
    }
    Ok(Arc::new(binaries))
}
