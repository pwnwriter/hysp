use crate::commands::hysp_cmd_helper::{ASCII, BAR, RESET};
use crate::engine::config::pkg_config_structure::PackageInfo;
use crate::engine::essetial::is_pkg_installed;
use crate::log::{abort, info};
use colored::Colorize;
use columns::Columns;
use spinoff::{spinners, Color, Spinner, Streams};

pub fn check_essentials(pkginfo: PackageInfo) {
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
            abort(&format!(
                "Dependency cycle detected aborting: {}",
                required_pkg
            ));
        }
    }

    spinner_deps.stop_and_persist(" Checking for dependencies  ", "Done");

    info(
        "No dependencies detected, proceeding .. ",
        colored::Color::Cyan,
    );
}

#[inline]
pub fn print_info(pkginfo: PackageInfo) {
    let pkg_bin_name = pkginfo.bin.name.bold().bright_red();
    let pkg_architecture = pkginfo.package.architecture.green();
    let pkg_desc = pkginfo.package.description.bold();
    let pkg_author = pkginfo.package.author.blue();
    let pkg_stars = pkginfo.package.stars.cyan();
    let pkg_version = pkginfo.package.version.blink();
    let pkg_size = pkginfo.package.size.yellow();
    let pkg_license = pkginfo.package.license.purple();
    let pkg_language = pkginfo.package.language.red();
    let pkg_hash = pkginfo.package.sha.bold().cyan();

    let package_information = Columns::from(vec![
        format!("{ASCII}").split('\n').collect::<Vec<&str>>(),
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
