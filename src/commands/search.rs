use crate::engine::config::read_config;
use crate::engine::helpers::print_info;
use crate::engine::{args::SearchArgs, config::pkg_config_structure::*};
use crate::log::info;
use anyhow::Result;
use reqwest;
use spinoff::{spinners, Color, Spinner, Streams};

pub async fn search_pkg(search_args: SearchArgs) -> Result<()> {
    let hysp_config = read_config().await?;
    let pkg_name = search_args.package.clone();
    let remote = hysp_config.source.remote.unwrap();
    let aarch = hysp_config.source.aarch.unwrap();

    let hysp_pkg_url = if remote.ends_with('/') {
        format!(
            "{}/{}/{}.toml",
            remote.trim_end_matches('/'),
            aarch,
            pkg_name
        )
    } else {
        format!("{}/{}/{}.toml", remote, aarch, pkg_name)
    };

    match get_status(&hysp_pkg_url).await {
        Ok(parsed_package_info) => {
            info("Package available", colored::Color::Blue);

            if !search_args.silent {
                print_info(parsed_package_info);
            }
        }
        Err(_err) => info("Package not available", colored::Color::Red),
    }

    Ok(())
}

async fn get_status(pkg_url: &str) -> Result<PackageInfo> {
    let mut spinner_status = Spinner::new_with_stream(
        spinners::Dots,
        "Checking for available packages ... ",
        Color::Green,
        Streams::Stderr,
    );
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10)) // Allow redirection limit = 10
        .danger_accept_invalid_certs(true) // allow http as well
        .build()?;

    let response = client.get(pkg_url).send().await?;
    let body = response.text().await?;
    let parsed_package_info: PackageInfo = toml::from_str(&body)?;
    spinner_status.stop_and_persist("Checking for available packages  ", "Done");
    Ok(parsed_package_info)
}
