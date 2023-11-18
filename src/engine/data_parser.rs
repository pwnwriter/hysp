use anyhow::{anyhow, Error};

pub async fn fetch_and_process_toml(pkg_name: &str) -> Result<String, Error> {
    const RAW_URL: &str = "https://raw.githubusercontent.com/bytehunt/seren-pkgs/main/data/";

    let pkg_toml_file_url = format!("{}", RAW_URL);

    let pkg_toml_file = format!("{}.toml", pkg_name);

    let response = reqwest::get(&pkg_toml_file_url).await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let toml_text = response.text().await?;
            Ok(toml_text)
        }
        _ => Err(anyhow!("No such package found")),
    }
}
