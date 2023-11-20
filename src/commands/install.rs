use crate::engine::data_parser::fetch_package_info;
use crate::engine::InstallArgs;
use spinoff::{spinners, Color, Spinner, Streams};
use std::env;
use std::fs::{self, File, Permissions};
use std::io::copy;
use std::os::unix::fs::PermissionsExt;

pub async fn download_pkgs(install_pkgs: InstallArgs) -> Result<(), anyhow::Error> {
    let pkgname = &install_pkgs.package;

    let toml_info = fetch_package_info(pkgname).await?;
    let binary_url = toml_info.source.url;

    download_binary(binary_url).await
}

async fn download_binary(binary_url: String) -> Result<(), anyhow::Error> {
    let spinner = Spinner::new_with_stream(
        spinners::Line,
        "Downloading binary please wait... ",
        Color::Yellow,
        Streams::Stderr,
    );
    let response = reqwest::get(&binary_url).await?;
    let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or_default();

    // Use current directory for file storage
    let current_dir = env::current_dir()?;
    let file_path = current_dir.join(fname);
    let mut dest = File::create(&file_path)?;

    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut dest)?;
    spinner.stop_and_persist("  ", "Done");

    let spinner = Spinner::new_with_stream(
        spinners::Line,
        "Setting up file permissions... ",
        Color::Green,
        Streams::Stderr,
    );
    // Set file permissions to be executable for the current user
    let permissions = Permissions::from_mode(0o755);
    fs::set_permissions(&file_path, permissions)?;

    spinner.stop_and_persist("  ", "Done");
    Ok(())
}
