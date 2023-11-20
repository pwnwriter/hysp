use crate::engine::pkg_info::PackageInfo;
use anyhow::{anyhow, Result};
use reqwest;
use reqwest::StatusCode;
use spinoff::{spinners, Color, Spinner};

fn build_package_toml_url(pkg_name: &str) -> String {
    let repo_url =
        std::env::var("SEREN_REPO_URL").unwrap_or_else(|_| "bytehunt/seren-pkgs".to_string());
    format!(
        "https://raw.githubusercontent.com/{}/main/data/{}.toml",
        repo_url, pkg_name
    )
}

pub async fn fetch_package_info(pkg_name: &str) -> Result<PackageInfo> {
    let pkg_toml_file_url = build_package_toml_url(pkg_name);

    let _spinner = Spinner::new(spinners::Dots, "Fetching package info... ", Color::Green);

    let response = reqwest::get(&pkg_toml_file_url)
        .await
        .map_err(|e| anyhow!("Failed to fetch package: {}", e))?;

    match response.status() {
        StatusCode::OK => {
            let toml_text = response.text().await?;
            let parsed_toml: PackageInfo = toml::from_str(&toml_text)?;
            // dbg!("{}", &parsed_toml);
            Ok(parsed_toml)
        }
        _ => Err(anyhow!("No such package found")),
    }
}
