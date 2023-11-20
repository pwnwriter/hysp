use crate::engine::package_information::PackageInfo;
use anyhow::{anyhow, Result};
use spinoff::{spinners, Color, Spinner};

pub async fn fetch_and_process_toml(pkg_name: &str) -> Result<PackageInfo> {
    let pkg_toml_file_url = build_package_toml_url(pkg_name)?;

    let _ = Spinner::new(spinners::Dots, "Fetching package info... ", Color::Green);
    let response = fetch_package_info(&pkg_toml_file_url).await?;

    println!();

    parse_package_response(response).await
}

// Read repo url from the envirovnment else use default one @bytehunt/seren-pkgs
fn build_package_toml_url(pkg_name: &str) -> Result<String> {
    let repo_url: String =
        std::env::var("SEREN_REPO_URL").unwrap_or("bytehunt/seren-pkgs".to_string());
    let raw_url = format!("https://raw.githubusercontent.com/{}/main/data/", repo_url);
    let pkg_toml_file = format!("{}.toml", pkg_name);
    Ok(format!("{}{}", raw_url, pkg_toml_file))
}

async fn fetch_package_info(url: &str) -> Result<reqwest::Response> {
    match reqwest::get(url).await {
        Ok(res) => Ok(res),
        Err(e) => Err(anyhow!("Failed to fetch package: {}", e)),
    }
}

async fn parse_package_response(response: reqwest::Response) -> Result<PackageInfo> {
    match response.status() {
        reqwest::StatusCode::OK => {
            let toml_text = response.text().await?;
            let parsed_toml: PackageInfo = toml::from_str(&toml_text)?;
            println!("{:?}", parsed_toml);
            Ok(parsed_toml)
        }
        _ => Err(anyhow!("No such package found")),
    }
}
