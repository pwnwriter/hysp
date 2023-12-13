use super::config::parse_local_config;
use crate::commands::ui::{ASCII, BAR, RESET};
use crate::engine::config::metadata_config_structure::MetadataInfo;
use crate::engine::config::pkg_config_structure::PackageInfo;
use crate::engine::msgx::info;
use anyhow::anyhow;
use anyhow::{Context, Result};
use colored::Colorize;
use columns::Columns;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use tokio::{fs::File, io::AsyncReadExt};

/// Formats a given field, returning "Not available" if empty.
///
/// # Arguments
///
/// * `field` - A string slice representing the field to be formatted.
///
/// # Returns
///
/// A formatted string with "Not available" for empty input or the original field.
///
#[inline]
fn format_field(field: &str) -> String {
    if field.is_empty() {
        "Not available".to_string()
    } else {
        field.to_string()
    }
}

/// Removes a trailing slash from a string path if it exists.
///
/// # Arguments
///
/// * `path` - A String representing the path to be modified.
///
/// # Returns
///
/// A string with the trailing slash removed (if present).
///
#[inline]
pub fn remove_trailing_slash(mut path: String) -> String {
    if path.ends_with('/') {
        path.pop();
    }
    path
}

/// Reads the contents of a file specified by `file_path` and returns them as a string.
///
/// # Arguments
///
/// * `file_path` - A string slice representing the path to the file.
///
/// # Returns
///
/// A Result containing either the file's contents as a string or an error.
///
#[inline]
pub async fn read_file_content(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)
        .await
        .with_context(|| format!("Failed to open file '{}'", file_path))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .with_context(|| format!("Failed to read file '{}'", file_path))?;

    Ok(contents)
}

/// Retrieves the system's architecture (either x86_64 or aarch64).
///
/// # Returns
///
/// A string representing the system architecture.
///
#[inline]
pub fn get_arch() -> String {
    let architecture = std::env::consts::ARCH.to_string();

    match architecture.as_str() {
        "x86_64" => "x86_64".to_string(),
        "aarch64" => "aarch64".to_string(),
        _ => {
            eprintln!("Error: Unsupported architecture");
            std::process::exit(1);
        }
    }
}

/// Prompts the user with a yes/no question and returns the user's response.
///
/// # Arguments
///
/// * `prompt` - A String containing the prompt message.
///
/// # Returns
///
/// A boolean value: `true` for yes, `false` for no.
///
#[inline]
pub fn prompt_yn(prompt: String) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim().to_lowercase();
        match trimmed.as_str() {
            "yes" | "y" => {
                info("Proceeding ...", colored::Color::Green);
                return true;
            }
            "no" | "n" => {
                info("Exitting .. ", colored::Color::Red);
                std::process::exit(1);
            }
            _ => {
                println!("Please enter 'y|es' or 'n|o'.");
            }
        }
    }
}

/// Prints package information based on PackageInfo struct.
///
/// # Arguments
///
/// * `pkginfo` - A PackageInfo struct containing package information.
///
#[inline]
pub fn print_package_info(pkginfo: PackageInfo) {
    // Needed
    let pkg_bin_name = &pkginfo.bin.name.bold().bright_red();
    let pkg_architecture = &pkginfo.package.architecture.green();
    let pkg_hash = &pkginfo.package.sha.bold().cyan();

    // Optional fields
    let pkg_desc = format_field(&pkginfo.package.description).bold().blue();
    let pkg_author = format_field(&pkginfo.package.author).cyan();
    let pkg_stars = format_field(&pkginfo.package.stars).yellow();
    let pkg_version = format_field(&pkginfo.package.version).green();
    let pkg_size = format_field(&pkginfo.package.size).bold().cyan();
    let pkg_license = format_field(&pkginfo.package.license).blue();
    let pkg_language = format_field(&pkginfo.package.language).cyan();

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

/// Prints metadata package information based on MetadataInfo struct and package name.
///
/// # Arguments
///
/// * `metadatainfo` - A MetadataInfo struct containing metadata information.
/// * `package_name` - A string slice representing the package name.
///
#[inline]
pub fn print_metadata_package_info(metadatainfo: MetadataInfo, package_name: &str) {
    if let Some(package) = metadatainfo
        .packages
        .iter()
        .find(|p| p.name == package_name)
    {
        // Needed fields
        let pkg_bin_name = &package.name;
        let pkg_architecture = &package.architecture;
        let pkg_hash = &package.sha.bold().cyan();

        // Optional fields
        let pkg_desc = format_field(&package.description).bold().blue();
        let pkg_author = format_field(&package.author).cyan();
        let pkg_stars = format_field(&package.stars).yellow();
        let pkg_version = format_field(&package.version).green();
        let pkg_size = format_field(&package.size).bold().cyan();
        let pkg_license = format_field(&package.license).blue();
        let pkg_language = format_field(&package.language).cyan();

        let package_information = Columns::from(vec![
            ASCII.split('\n').collect::<Vec<&str>>(),
            vec![
                &format!("Package: {}", pkg_bin_name),
                &format!("Architecture: {}", pkg_architecture),
                &format!("Stars: {}", pkg_stars),
                &format!("Version: {}", pkg_version),
                &format!("Author: {}", pkg_author),
                &format!("Size: {}", pkg_size),
                &format!("Desc: {}", pkg_desc),
                &format!("Hash: {}", pkg_hash),
                &format!("Language: {}", pkg_language),
                &format!("License: {}", pkg_license),
            ],
        ])
        .set_tabsize(15)
        .make_columns();
        println!("{}", RESET);
        println!("{}", BAR.purple());
        println!("{}", package_information);
        println!("{}", BAR.purple());
    } else {
        info(
            &format!("No such package found as: {}", package_name),
            colored::Color::Cyan,
        );
    }
}

/// Fetches local configuration data.
///
/// # Returns
///
/// A Result containing a tuple of strings representing remote, data directory,
/// binary directory, and metadata link, or an error.
///
#[inline]
pub async fn local_config() -> Result<(String, String, String, String, String)> {
    let hysp_config = parse_local_config().await?;

    let hysp_remote = remove_trailing_slash(
        hysp_config
            .source
            .remote
            .ok_or_else(|| anyhow!("Couldn't get data directory"))?
            .to_string(),
    );

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

    let hysp_metadata = remove_trailing_slash(
        hysp_config
            .source
            .metadata
            .ok_or_else(|| anyhow!("Couldn't get metadata link"))?
            .to_string(),
    );

    let system_arch = hysp_config
        .source
        .aarch
        .ok_or_else(|| anyhow!("Couldn't get aarch"))?
        .to_string();

    Ok((
        hysp_remote,
        hysp_data_dir,
        hysp_bin_dir,
        hysp_metadata,
        system_arch,
    ))
}

/// Creates a directory at the specified path if it does not already exist.
///
/// # Arguments
///
/// * `path` - A string slice representing the path of the directory to create.
///
#[inline]
pub fn create_directory_if_not_exists(path: &str) {
    if let Err(err) = std::fs::create_dir_all(path) {
        eprintln!("Error creating directory '{}': {}", path, err);
    }
}

/// Checks the hash of a file against an expected hash.
///
/// # Arguments
///
/// * `pkgname` - A String containing the file name.
/// * `expected_hash` - A String containing the expected hash.
///
/// # Returns
///
/// A Result containing a boolean value: `true` if the actual hash matches the expected hash,
/// `false` otherwise, or an error.
///
#[inline]
pub async fn check_hash(filename: String, expected_hash: String) -> Result<bool, io::Error> {
    let mut file = File::open(filename).await?;

    let mut hasher = Sha256::new();

    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let actual_hash = format!("{:x}", hasher.finalize());

    Ok(actual_hash == expected_hash)
}

/// Checks if a package is installed.
///
/// # Arguments
///
/// * `pkg_name` - A string slice representing the name of the package.
///
/// # Returns
///
/// A boolean value indicating whether the package is installed.
///
#[inline]
pub fn is_pkg_installed(pkg_name: &str) -> bool {
    let output = Command::new("which")
        .arg(pkg_name)
        .output()
        .expect("Failed to execute 'which' command");

    output.status.success()
}

/// Removes a file and prints a message indicating success or failure.
///
/// # Arguments
///
/// * `file_path` - A string slice representing the path of the file to be removed.
///
#[inline]
pub fn remove_and_print(file_path: &str) {
    if let Err(err) = fs::remove_file(file_path) {
        eprintln!("Error removing {}: {}", file_path, err);
    } else {
        info(&format!("Removed : {}", file_path), colored::Color::Cyan);
    }
}
