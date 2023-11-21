use crate::engine::parser::fetch_package_info;
use crate::engine::InstallArgs;
use anyhow::Result;
use spinoff::{spinners, Color, Spinner, Streams};
use std::process::Command;

pub async fn check_conflicts(install_pkgs: InstallArgs) -> Result<()> {
    let pkgname = &install_pkgs.package;

    let toml_info = fetch_package_info(pkgname).await?;
    if let Some(conflicts) = toml_info.package.conditions.and_then(|c| c.conflicts) {
        let spinner = Spinner::new_with_stream(
            spinners::Line,
            "Checking for conflicts ...",
            Color::Blue,
            Streams::Stderr,
        );

        for conflict_pkg in conflicts {
            if is_pkg_installed(&conflict_pkg) {
                spinner.stop_and_persist("  ", "Done");
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
        let spinner = Spinner::new_with_stream(
            spinners::Line,
            "Checking for dependencies  ...",
            Color::Green,
            Streams::Stderr,
        );

        for dependency_pkg in dependencies {
            if !is_pkg_installed(&dependency_pkg) {
                spinner.stop_and_persist("  ", "Done");
                return Err(anyhow::anyhow!("Dependency '{}' not found", dependency_pkg));
            }
        }
    }

    Ok(())
}

// pub async fn print_maintainer_info(pkgname: &str) -> Result<()> {
//     let toml_info = fetch_package_info(pkgname).await?;
//
//     if let Some(maintainer) = toml_info.maintainer {
//         println!("Maintainer Name: {}", maintainer.name);
//         println!("Maintainer Email: {}", maintainer.email);
//         if let Some(link) = maintainer.link {
//             println!("Maintainer Link: {}", link);
//         }
//         // Add more fields if needed
//     } else {
//         println!("No maintainer information found for {}", pkgname);
//     }
//
//     Ok(())
// }

fn is_pkg_installed(pkg_name: &str) -> bool {
    let output = Command::new("which")
        .arg(pkg_name)
        .output()
        .expect("Failed to execute 'which' command");

    output.status.success()
}
