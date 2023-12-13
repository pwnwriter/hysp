use crate::engine::{
    args::InstallArgs,
    helpers::{
        check_hash, create_directory_if_not_exists, is_pkg_installed, local_config,
        print_package_info, prompt_yn, remove_trailing_slash,
    },
    msgx::info,
    request::{download_and_parse_package, download_as_byte},
};
use anyhow::Result;
use async_recursion::async_recursion;
use spinoff::{spinners, Color, Spinner, Streams};
use std::fs::remove_file;
use std::path::Path;
use tokio::task;

pub async fn install_packages(install_pkgs: InstallArgs) -> Result<()> {
    let mut tasks = vec![];

    for package_name in install_pkgs.packages.clone() {
        let install_pkgs_clone = install_pkgs.clone();
        let task = task::spawn(async move {
            if let Err(e) = install_pkg(&install_pkgs_clone, &package_name, false).await {
                eprint!("{}", e);
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("Task failed: {}", e);
        }
    }

    Ok(())
}

#[async_recursion]
pub async fn install_pkg(
    install_pkgs: &InstallArgs,
    package_name: &str,
    is_dependency: bool,
) -> Result<()> {
    let (hysp_remote, hysp_data_dir, hysp_bin_dir, _hysp_metadata, _architecture) =
        match local_config().await {
            Ok((remote, data_dir, bin_dir, metadata, architecture)) => {
                (remote, data_dir, bin_dir, metadata, architecture)
            }
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

    create_directory_if_not_exists(&hysp_bin_dir);
    create_directory_if_not_exists(&hysp_data_dir);

    let package_data_location = format!(
        "{}/{}.toml",
        remove_trailing_slash(hysp_data_dir.clone()),
        package_name
    );

    if !install_pkgs.force && Path::new(&package_data_location).exists() {
        info(
            &format!(
                "There's already a package exist as: {} ",
                package_data_location
            ),
            colored::Color::Cyan,
        );
        prompt_yn("Would you like to overwrite ? (y/n)".to_string());
    }

    let package_download_url = format!(
        "{}/{}.toml",
        remove_trailing_slash(hysp_remote),
        package_name
    );

    match download_and_parse_package(&package_download_url, &package_data_location).await {
        Ok(parsed_info) => {
            let binary_path = format!(
                "{}/{}",
                remove_trailing_slash(hysp_bin_dir),
                parsed_info.bin.name
            );

            let dependency_names: Vec<_> = parsed_info.package.conditions.requires.to_vec();

            if !dependency_names.is_empty() {
                let dependencies_str = dependency_names.join(", ");
                info(
                    &format!(
                        "Dependency cycle detected installing dependencies: {}",
                        dependencies_str
                    ),
                    colored::Color::Cyan,
                );
            }

            for dep in parsed_info.package.conditions.requires.clone() {
                if !is_pkg_installed(&dep) {
                    install_pkg(install_pkgs, &dep, true).await?;
                }
            }

            if install_pkgs.packages.len() < 2 && !is_dependency && !install_pkgs.quiet {
                print_package_info(parsed_info.clone());
            }

            download_as_byte(&parsed_info.package.source, &binary_path).await?;

            let mut spinner_hash = Spinner::new_with_stream(
                spinners::Dots,
                "Validating hash ... ",
                Color::Green,
                Streams::Stderr,
            );
            let check_result =
                check_hash(binary_path.clone(), parsed_info.package.sha.clone()).await;

            if let Ok(result) = check_result {
                if !result {
                    if let Err(e) = remove_file(&binary_path) {
                        eprintln!("Failed to remove file: {}", e);
                    }
                    if let Err(e) = remove_file(&package_data_location) {
                        eprintln!("Failed to remove package data: {}", e);
                    }
                }
            } else {
                eprintln!("Error checking hash: {}", check_result.err().unwrap());
            }
            spinner_hash.stop_and_persist("Validating hash  ", "Done");
        }
        Err(e) => {
            eprint!("{}", e);
        }
    };

    Ok(())
}
