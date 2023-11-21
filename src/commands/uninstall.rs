use crate::commands::hysp_helpers::read_local_file;
use crate::engine::dirs::*;
use crate::engine::essentials::print_package_info;
use crate::engine::pkg_info::PackageInfo;
use crate::engine::RemoveArgs;
use anyhow::Context;
use spinoff::{spinners, Color, Spinner, Streams};
use std::fs;

pub async fn remove_pkgs(uninstall_pkgs: RemoveArgs) -> Result<(), anyhow::Error> {
    let pkgname = uninstall_pkgs.package;
    let package_info = get_local_toml(&pkgname).await?;

    if !uninstall_pkgs.silent {
        let _ = print_package_info(&pkgname).await;
    }

    remove_binary_file(&package_info.bin.name)?;

    Ok(())
}

pub async fn get_local_toml(pkgname: &str) -> Result<PackageInfo, anyhow::Error> {
    let package_file = format!("{}/{}.toml", HYSP_DATA_DIR.to_string_lossy(), pkgname);
    let toml_text = read_local_file(&package_file)
        .await
        .with_context(|| format!("Failed to read TOML file: {}", package_file))?;
    let parsed_toml: PackageInfo = toml::from_str(&toml_text)?;
    Ok(parsed_toml)
}

fn remove_binary_file(binary_name: &str) -> Result<(), anyhow::Error> {
    let mut spinner_01 = Spinner::new_with_stream(
        spinners::Arc,
        "Removing package binary and data  ...",
        Color::Yellow,
        Streams::Stderr,
    );
    let bin_file_path = format!("{}/{}", HYSP_BIN_DIR.to_string_lossy(), binary_name);
    let bin_data_file_path = format!("{}/{}.toml", HYSP_DATA_DIR.to_string_lossy(), binary_name);
    fs::remove_file(&bin_file_path)
        .with_context(|| format!("Failed to remove binary file: {}", bin_file_path))?;

    fs::remove_file(&bin_data_file_path)
        .with_context(|| format!("Failed to remove binary data file: {}", bin_data_file_path))?;
    spinner_01.stop_and_persist("  ", "Done");
    Ok(())
}
