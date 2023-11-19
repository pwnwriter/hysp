use crate::engine::package_information::PackageInfo;
use anyhow::{anyhow, Result};
use spinoff::{spinners, Color, Spinner};

pub async fn fetch_and_process_toml(pkg_name: &str) -> Result<PackageInfo> {
    const RAW_URL: &str = "https://raw.githubusercontent.com/bytehunt/seren-pkgs/main/data/";

    let pkg_toml_file = format!("{}.toml", pkg_name);
    let pkg_toml_file_url = format!("{}{}", RAW_URL, pkg_toml_file);
    let _ = Spinner::new(spinners::Dots, "Fetching package info... ", Color::Green); 
    // TODO: Implement some fancy style for spinner
    let response = match reqwest::get(&pkg_toml_file_url).await {
        Ok(res) => res,
        Err(e) => return Err(anyhow!("Failed to fetch package: {}", e)),
    };
    println!();
    match response.status() {
        reqwest::StatusCode::OK => {
            let toml_text = match response.text().await {
                Ok(text) => text,
                Err(e) => return Err(anyhow!("Failed to parse response {}", e)),
            };

            let parsed_toml: PackageInfo = toml::from_str(&toml_text)?;

            println!("{:?}", parsed_toml);

            Ok(parsed_toml)
        }
        _ => Err(anyhow!("No such package found")),
    }
}
