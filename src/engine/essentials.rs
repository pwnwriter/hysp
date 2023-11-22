use crate::engine::InstallArgs;
use crate::{
    commands::hysp_helpers::{ASCII, BAR},
    engine::parser::fetch_package_info,
};
use anyhow::Result;
use colored::Colorize;
use columns::Columns;
use spinoff::{spinners, Color, Spinner, Streams};
use std::process::Command;

pub async fn check_conflicts(install_pkgs: InstallArgs) -> Result<()> {
    let pkgname = &install_pkgs.package;

    let toml_info = fetch_package_info(pkgname).await?;
    let mut spinner_02 = Spinner::new_with_stream(
        spinners::Runner,
        "Checking for conflicts ...",
        Color::Blue,
        Streams::Stderr,
    );
    if let Some(conflicts) = toml_info.package.conditions.and_then(|c| c.conflicts) {
        for conflict_pkg in conflicts {
            spinner_02.stop_and_persist("  ", "Done");
            if is_pkg_installed(&conflict_pkg) {
                return Err(anyhow::Error::msg(format!(
                    "Confliction detected aborting .. {conflict_pkg}"
                )));
            }
        }
    }

    Ok(())
}

pub async fn check_dependencies(install_pkgs: &InstallArgs) -> Result<()> {
    let pkgname = &install_pkgs.package;

    let toml_info = fetch_package_info(pkgname).await?;
    if let Some(dependencies) = toml_info.package.conditions.and_then(|c| c.dependencies) {
        let mut spinner_04 = Spinner::new_with_stream(
            spinners::Line,
            "Checking for dependencies  ...",
            Color::Green,
            Streams::Stderr,
        );

        for dependency_pkg in dependencies {
            if !is_pkg_installed(&dependency_pkg) {
                spinner_04.stop_and_persist("  ", "Done");
                return Err(anyhow::anyhow!("Dependency '{}' not found", dependency_pkg));
            }
        }
    }

    Ok(())
}

pub async fn print_package_info(pkgname: &str) -> Result<()> {
    let toml_info = fetch_package_info(pkgname).await?;

    let main_tainer = toml_info.maintainer.name.purple();
    let main_tainer_email = toml_info.maintainer.email.green();
    let pkg_version = toml_info.package.version.cyan();
    let pkg_desc = toml_info.package.description.bold();
    let pkg_license = toml_info.package.license.yellow();
    let pkg_size = toml_info.package.size.yellow();

    let package_information = Columns::from(vec![
        format!("{ASCII}").split('\n').collect::<Vec<&str>>(),
        vec![
            &format!("Package: {pkgname}"),
            &format!("Version: {pkg_version}"),
            &format!("Maintainer: {main_tainer}"),
            &format!("Email: {main_tainer_email}"),
            &format!("Size: {pkg_size}"),
            &format!("Desc: {pkg_desc}"),
            &format!("License: {pkg_license}"),
        ],
    ])
    .set_tabsize(15)
    .make_columns();
    println!("{}", package_information);
    println!("{}", BAR);
    Ok(())
}

fn is_pkg_installed(pkg_name: &str) -> bool {
    let output = Command::new("which")
        .arg(pkg_name)
        .output()
        .expect("Failed to execute 'which' command");

    output.status.success()
}
