use crate::commands::seren_helpers::{ASCII, BAR};
use crate::engine::pkg_info::available_packages::Available;
use crate::engine::{parser::build_package_toml_url, SearchArgs};
use anyhow::anyhow;
use colored::Colorize;
use columns::Columns;
use reqwest::StatusCode;
use spinoff::{spinners, Color, Spinner, Streams};

pub async fn fetch_available(search_args: SearchArgs) -> Result<(), anyhow::Error> {
    let _ = parse_available_toml(&search_args.package).await;
    Ok(())
}

pub async fn parse_available_toml(pkg_name: &str) -> Result<(), anyhow::Error> {
    let available_toml_url = build_package_toml_url(None);

    let spinner = Spinner::new_with_stream(
        spinners::Line,
        "Fetching available packages info ... ",
        Color::Yellow,
        Streams::Stderr,
    );

    let response = reqwest::get(&available_toml_url)
        .await
        .map_err(|e| anyhow!("Failed to fetch package: {}", e))?;

    match response.status() {
        StatusCode::OK => {
            let toml_text = response.text().await?;
            let parsed_toml: Available = toml::from_str(&toml_text)?;

            spinner.stop_and_persist("  ", "Done");

            for package in parsed_toml.packages {
                if package.name == pkg_name {
                    let pkg_name = package.name.red();
                    let pkg_size = package.binary_size.cyan();
                    let pkg_maintainer = package.maintainer.yellow();
                    let pkg_maintainers_mail = package.email.purple();
                    let pkg_license = package.license.blue();
                    let pkg_homepage = package.homepage.red();
                    let pkg_desc = package.description.bold();
                    let pkg_version = package.version.green();

                    let package_information = Columns::from(vec![
                        format!("{ASCII}").split('\n').collect::<Vec<&str>>(),
                        vec![
                            &format!("Package: {pkg_name}"),
                            &format!("Size: {pkg_size}"),
                            &format!("Version: {pkg_version}"),
                            &format!("Maintainer: {pkg_maintainer}"),
                            &format!("Email: {pkg_maintainers_mail}"),
                            &format!("Size: {pkg_size}"),
                            &format!("Desc: {pkg_desc}"),
                            &format!("Homepage: {pkg_homepage}"),
                            &format!("License: {pkg_license}"),
                        ],
                    ])
                    .set_tabsize(15)
                    .make_columns();
                    println!("{}", package_information);
                    println!("{}", BAR);
                }
            }

            Ok(())
        }
        _ => Err(anyhow!("No database found")),
    }
}
