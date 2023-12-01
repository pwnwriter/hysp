use crate::{
    engine::{
        args::RemoveArgs,
        config::{pkg_config_structure::PackageInfo, read_config},
    },
    log::info,
};
use spinoff::{spinners, Color, Spinner, Streams};
use std::{fs::remove_file, io::ErrorKind};

use anyhow::{anyhow, Result};

use super::hysp_cmd_helper::read_file_content;

pub async fn remove_pkg(remove_pkg: RemoveArgs) -> Result<()> {
    let hysp_config = read_config().await?;

    let mut spinner_removepkg = Spinner::new_with_stream(
        spinners::Dots,
        "Getting pkg info ... ",
        Color::Green,
        Streams::Stderr,
    );

    println!();

    let hysp_data_dir = remove_trailing_slash(
        hysp_config
            .local
            .data
            .ok_or_else(|| anyhow!("Couldn't get data directory"))?
            .to_string_lossy()
            .to_string(),
    );

    let hysp_bin_dir = remove_trailing_slash(
        hysp_config
            .local
            .bin
            .ok_or_else(|| anyhow!("Couldn't get binary directory"))?
            .to_string_lossy()
            .to_string(),
    );

    let pkg_name = remove_pkg.package;
    let pkg_toml_location = format!("{}/{}.toml", hysp_data_dir, pkg_name);

    let contents = read_file_content(&pkg_toml_location)
        .await
        .map_err(|err| err)?;

    let parsed_config_toml = toml::from_str::<PackageInfo>(&contents)?;

    let package_binary = parsed_config_toml.bin.name;
    let pkg_binary_location = format!("{}/{}", hysp_bin_dir, package_binary);
    let pkg_binary_toml_location = format!("{}/{}.toml", hysp_data_dir, package_binary);

    let mut formatted_dependencies: Vec<String> = Vec::new();

    for dependency in &parsed_config_toml.package.conditions.requires {
        formatted_dependencies.push(dependency.to_string());
    }

    let dependencies_str = formatted_dependencies.join(",");
    
    println!();

    info(
        &format!("Dependencies detected, removing: {}", dependencies_str),
        colored::Color::Cyan,
    );

    for dependency in &parsed_config_toml.package.conditions.requires {
        let dep_binary_location = format!("{}/{}", hysp_bin_dir, dependency);
        let dep_binary_toml_location = format!("{}/{}.toml", hysp_data_dir, dependency);
        remove_file_silent(&dep_binary_location);
        remove_file_silent(&dep_binary_toml_location);
    }

    remove_file_silent(&pkg_binary_location);
    remove_file_silent(&pkg_binary_toml_location);
    spinner_removepkg.stop_and_persist("Removed packages  ", "Done");
    Ok(())
}

fn remove_file_silent(file_path: &str) {
    if let Err(err) = remove_file(file_path) {
        match err.kind() {
            ErrorKind::NotFound => {
                println!("No such package installed as : {}", file_path);
                std::process::exit(1);
            }
            _ => {
                info(
                    &format!("Error removing file {}: {}", file_path, err),
                    colored::Color::Cyan,
                );
            }
        }
    }
}

pub fn remove_trailing_slash(mut path: String) -> String {
    if path.ends_with('/') {
        path.pop();
    }
    path
}
