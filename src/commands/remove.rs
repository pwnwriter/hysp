use crate::{
    engine::{args::RemoveArgs, config::read_config},
    log::info,
};
use spinoff::{spinners, Color, Spinner, Streams};
use std::io::{Error, ErrorKind};

use anyhow::{anyhow, Result};

pub async fn remove_pkg(remove_pkg: RemoveArgs) -> Result<()> {
    let hysp_config = read_config().await?;

    let mut spinner_removepkg = Spinner::new_with_stream(
        spinners::Dots,
        "Getting pkg info ... ",
        Color::Green,
        Streams::Stderr,
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

    let pkg_name = remove_pkg.package;

    let pkg_binary_location = format!("{}/{}", hysp_bin_dir, pkg_name);
    let pkg_toml_location = format!("{}/{}.toml", hysp_data_dir, pkg_name);

    if remove_file(&pkg_binary_location).is_err() | remove_file(&pkg_toml_location).is_err() {
    } else {
        println!();
        info(
            &format!("To remove: {}", &pkg_binary_location),
            colored::Color::Cyan,
        );
        spinner_removepkg.stop_and_persist("Removed pkg successfully  ", "Done");
    }

    Ok(())
}

fn remove_file(file_path: &str) -> Result<(), Error> {
    if let Err(err) = std::fs::remove_file(file_path) {
        match err.kind() {
            ErrorKind::NotFound => {
                println!("No such package installed as : {}", file_path);
                std::process::exit(1);
            }
            _ => return Err(err),
        }
    }
    Ok(())
}

pub fn remove_trailing_slash(mut path: String) -> String {
    if path.ends_with('/') {
        path.pop();
    }
    path
}
