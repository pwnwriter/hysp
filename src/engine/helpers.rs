use super::args::InstallArgs;
use super::config::read_config;
use crate::commands::hysp_cmd_helper::{ASCII, BAR, RESET};
use crate::commands::install::{download_binary, fetch_pkg};
use crate::engine::config::pkg_config_structure::PackageInfo;
use crate::engine::essetial::is_pkg_installed;
use crate::log::{abort, info};
use anyhow::Result;
use colored::Colorize;
use columns::Columns;
use spinoff::{spinners, Color, Spinner, Streams};

pub async fn check_essentials(pkginfo: PackageInfo, install_pkgs: InstallArgs) -> Result<()> {
    let mut spinner_conflicts = Spinner::new_with_stream(
        spinners::Dots,
        "Checking for conflicts ... ",
        Color::Green,
        Streams::Stderr,
    );
    for conflict_pkg in &pkginfo.package.conditions.conflicts {
        if conflict_pkg == &pkginfo.package.name {
            continue;
        }
        if is_pkg_installed(conflict_pkg) {
            abort(&format!("Conflict detected aborting: {}", conflict_pkg));
        }
    }
    spinner_conflicts.stop_and_persist(" Checking for conflicts  ", "Done");
    info(
        "No conflicts detected, proceeding .. ",
        colored::Color::Cyan,
    );

    let mut spinner_deps = Spinner::new_with_stream(
        spinners::Arc,
        "Checking for dependencies ... ",
        Color::Green,
        Streams::Stderr,
    );
    for required_pkg in pkginfo.package.conditions.requires {
        if !is_pkg_installed(&required_pkg) {
            info(
                &format!(
                    "Dependency cycle : {} detected, installing depedent binaries",
                    required_pkg,
                ),
                colored::Color::Cyan,
            );

            let _ = install_dependencies(&required_pkg, install_pkgs.clone()).await;
        } else {
            info(
                "No dependencies detected, proceeding .. ",
                colored::Color::Cyan,
            );
        }
    }

    spinner_deps.stop_and_persist("Checking for dependencies  ", "Done");
    Ok(())
}

#[inline]
pub fn print_info(pkginfo: PackageInfo) {
    // Needed
    let pkg_bin_name = pkginfo.bin.name.bold().bright_red();
    let pkg_architecture = pkginfo.package.architecture.green();
    let pkg_hash = pkginfo.package.sha.bold().cyan();

    // Optional
    let pkg_desc = if pkginfo.package.description.is_empty() {
        "Not available".to_string()
    } else {
        pkginfo.package.description.clone()
    };

    let pkg_author = if pkginfo.package.author.is_empty() {
        "Not available".to_string()
    } else {
        pkginfo.package.author.clone()
    };

    let pkg_stars = if pkginfo.package.stars.is_empty() {
        "Not available".to_string()
    } else {
        pkginfo.package.stars.clone()
    };

    let pkg_version = if pkginfo.package.version.is_empty() {
        "Not available".to_string()
    } else {
        pkginfo.package.version.clone()
    };

    let pkg_size = if pkginfo.package.size.is_empty() {
        "Not available".to_string()
    } else {
        pkginfo.package.size.clone()
    };

    let pkg_license = if pkginfo.package.license.is_empty() {
        "Not available".to_string()
    } else {
        pkginfo.package.license.clone()
    };

    let pkg_language = if pkginfo.package.language.is_empty() {
        "Not available".to_string()
    } else {
        pkginfo.package.language.clone()
    };

    let package_information = Columns::from(vec![
        ASCII.split('\n').collect::<Vec<&str>>(),
        vec![
            &format!("Package: {pkg_bin_name}"),
            &format!("Architecture: {pkg_architecture}"),
            &format!("Stars: {pkg_stars}"),
            &format!("Version: {pkg_version}"),
            &format!("Author: {pkg_author}"),
            &format!("Size: {pkg_size}"),
            &format!("Desc: {pkg_desc}"),
            &format!("Hash: {pkg_hash}"),
            &format!("Language: {pkg_language}"),
            &format!("License: {pkg_license}"),
        ],
    ])
    .set_tabsize(15)
    .make_columns();
    println!("{}", RESET); // RESET terminal colors
    println!("{}", BAR.purple());
    println!("{}", package_information);
    println!("{}", BAR.purple());
}

#[inline]
pub fn create_directory_if_not_exists(path: &str) {
    if let Err(err) = std::fs::create_dir_all(path) {
        eprintln!("Error creating directory '{}': {}", path, err);
    }
}

#[inline]
pub async fn install_dependencies(pkg_name: &str, install_pkgs: InstallArgs) -> Result<()> {
    let hysp_config = read_config().await?;

    let remote = hysp_config.source.remote.unwrap();

    let aarch = hysp_config.source.aarch.unwrap();

    let home_location = hysp_config
        .local
        .home
        .unwrap()
        .to_string_lossy()
        .to_string();
    let bin_location = hysp_config.local.bin.unwrap().to_string_lossy().to_string();
    let data_location = hysp_config
        .local
        .data
        .unwrap()
        .to_string_lossy()
        .to_string();

    create_directory_if_not_exists(&home_location);
    create_directory_if_not_exists(&bin_location);
    create_directory_if_not_exists(&data_location);

    let pkg_url = if remote.ends_with('/') {
        format!(
            "{}/{}/{}.toml",
            remote.trim_end_matches('/'),
            aarch,
            pkg_name
        )
    } else {
        format!("{}/{}/{}.toml", remote, aarch, pkg_name)
    };

    let package_config = fetch_pkg(
        pkg_url,
        data_location.clone(),
        pkg_name.to_string(),
        install_pkgs,
    )
    .await;
    match package_config {
        Ok(package_config) => {
            let binary_url = package_config.package.source.clone();
            if let Err(err) = download_binary(
                binary_url,
                package_config.bin.name.clone(),
                package_config.package.sha,
                bin_location,
            )
            .await
            {
                eprintln!("Error downloading binary: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Error fetching package: {}", err);
        }
    }

    Ok(())
}
