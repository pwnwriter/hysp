use crate::engine::config::read_config;
use crate::{commands::remove::remove_trailing_slash, log::info};
use anyhow::{anyhow, Result};
use std::fs;

pub async fn list_pkgs() -> Result<()> {
    let hysp_config = read_config().await?;

    let hysp_bin_dir = remove_trailing_slash(
        hysp_config
            .local
            .bin
            .ok_or_else(|| anyhow!("Couldn't get binary directory"))?
            .to_string_lossy()
            .to_string(),
    );

    match list_files_in_directory(&hysp_bin_dir) {
        Ok(files) => {
            info(
                &format!("Installed pkgs in: {} ", hysp_bin_dir),
                colored::Color::Cyan,
            );
            for file in &files {
                println!("{}", file);
            }
        }
        Err(err) => {
            eprintln!("Error listing files in {}: {}", hysp_bin_dir, err);
        }
    }

    Ok(())
}

pub fn list_files_in_directory(directory: &str) -> Result<Vec<String>, std::io::Error> {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(e) => return Err(e),
    };

    let mut files = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().into_string().ok() {
                files.push(file_name);
            }
        }
    }

    Ok(files)
}
