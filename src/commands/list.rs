use crate::engine::args::ListArgs;
use crate::engine::config::parse_metadata_info;
use crate::engine::config::pkg_config_structure::PackageInfo;
use crate::engine::helpers::{
    local_config, print_metadata_package_info, print_package_info, read_file_content,
};
use crate::engine::msgx::{abort, info};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

const TOML_EXTENSION: &str = ".toml";

pub async fn list_pkgs(pkg_list_args: ListArgs) -> Result<()> {
    let (_hysp_remote, hysp_data_dir, _hysp_bin_dir, hysp_metadata) = match local_config().await {
        Ok((remote, data_dir, bin_dir, metadata)) => (remote, data_dir, bin_dir, metadata),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    match (pkg_list_args.available, pkg_list_args.installed) {
        (true, false) => {
            print_available_pkg(&hysp_metadata, pkg_list_args.verbose).await?;
        }
        (false, true) => {
            print_installed_pkgs(&hysp_data_dir, pkg_list_args.verbose).await?;
        }
        _ => {
            abort("No such arg available");
        }
    }

    Ok(())
}

async fn print_available_pkg(metadata_toml: &str, verbose: bool) -> Result<(), anyhow::Error> {
    let metadata_toml_info = parse_metadata_info(metadata_toml).await?;
    info("Available packages in metadata", colored::Color::Green);
    if verbose {
        for package in metadata_toml_info.packages.clone() {
            print_metadata_package_info(metadata_toml_info.clone(), &package.name);
        }
    } else {
        for package in metadata_toml_info.packages {
            println!("{}", package.name);
        }
    }

    Ok(())
}

async fn print_installed_pkgs(hysp_data_dir: &str, verbose: bool) -> Result<()> {
    let packages: Vec<(String, String)> = iterate_over_package_files(hysp_data_dir).collect();

    if packages.is_empty() {
        info("No packages installed via hysp", colored::Color::Red);
    } else {
        info("Installed packages ", colored::Color::Blue);
        for (file_path, file_name) in packages {
            if verbose {
                get_package_info(&file_path).await?;
            } else {
                println!("{}", file_name);
            }
        }
    }
    Ok(())
}

fn iterate_over_package_files(file_path: &str) -> impl Iterator<Item = (String, String)> + '_ {
    fs::read_dir(file_path)
        .expect("Failed to read directory")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter_map(extract_file_info)
}

fn extract_file_info(entry: PathBuf) -> Option<(String, String)> {
    let file_name = entry.file_name()?.to_str()?;
    let stripped_name = Path::new(file_name).file_stem()?.to_str()?;
    let (file_path, file_name) = if stripped_name.ends_with(TOML_EXTENSION) {
        let file_path = entry.to_string_lossy().to_string();
        let file_name = stripped_name
            .strip_suffix(TOML_EXTENSION)
            .expect("Expected TOML extension")
            .to_string();
        (file_path, file_name)
    } else {
        let file_path = entry.to_string_lossy().to_string();
        let file_name = stripped_name.to_string();
        (file_path, file_name)
    };
    Some((file_path, file_name))
}

async fn get_package_info(pkg_file: &str) -> Result<()> {
    let package_toml = read_file_content(pkg_file)
        .await
        .context("Failed to read package file")?;
    let parsed_package_info: PackageInfo =
        toml::from_str(&package_toml).context("Failed to parse package TOML")?;

    print_package_info(parsed_package_info);

    Ok(())
}
