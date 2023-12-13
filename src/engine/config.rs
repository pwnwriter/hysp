use crate::commands::ui::BAR;
use crate::engine::config::metadata_config_structure::MetadataInfo;
use crate::engine::config::pkg_config_structure::PackageInfo;
use crate::engine::helpers::{get_arch, read_file_content};
use crate::engine::msgx::info;
use anyhow::{anyhow, Result};
use colored::Colorize;
use local_config_structure::LocalConfig;
use reqwest;

pub mod local_config_structure {
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct LocalConfig {
        pub source: Source,
        pub local: Local,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Source {
        pub remote: Option<String>,
        pub metadata: Option<String>,
        pub aarch: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Local {
        pub home: Option<PathBuf>,
        pub bin: Option<PathBuf>,
        pub data: Option<PathBuf>,
    }
}

pub mod pkg_config_structure {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PackageInfo {
        pub bin: Bin,
        pub package: Package,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Bin {
        pub name: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Package {
        pub architecture: String,
        pub name: String,
        pub description: String,
        pub author: String,
        pub repo: String,
        pub stars: String,
        pub version: String,
        pub updated: String,
        pub size: String,
        pub sha: String,
        pub source: String,
        pub language: String,
        pub license: String,
        pub conditions: Conditions,
        pub metadata: Metadata,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Conditions {
        pub conflicts: Vec<String>,
        pub requires: Vec<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Metadata {
        pub keywords: Vec<String>,
        pub categories: Vec<String>,
    }
}

pub mod metadata_config_structure {

    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MetadataInfo {
        pub packages: Vec<Package>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Package {
        pub architecture: String,
        pub name: String,
        pub description: String,
        pub author: String,
        pub repo: String,
        pub stars: String,
        pub version: String,
        pub updated: String,
        pub size: String,
        pub bsum: String,
        pub sha: String,
        pub source: String,
        pub language: String,
        pub license: String,
    }
}

/// Parse hysp user config
#[inline]
pub async fn parse_local_config() -> Result<LocalConfig> {
    let config_file_path = dirs::config_dir()
        .ok_or_else(|| anyhow!("Config directory not found"))?
        .join("hysp/config.toml");

    let arch = get_arch();

    let contents = match read_file_content(&config_file_path.to_string_lossy()).await {
        Ok(contents) => {
            println!("{}", BAR.bold().purple());
            info(
                &format!(
                    "Using custom config from: {} ",
                    config_file_path.to_string_lossy(),
                ),
                colored::Color::Cyan,
            );
            println!("{}", BAR.bold().purple());
            contents
        }
        Err(_) => {
            println!("{}", BAR.bold().purple());
            info("Using default configuration.", colored::Color::Green);
            println!("{}", BAR.bold().purple());
            let home = match std::env::var("HOME") {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Error: HOME environment variable not found");
                    std::process::exit(0);
                }
            };
            format!(
                r#"
                [source]
                remote = "https://raw.githubusercontent.com/metis-os/hysp-pkgs/main/data/x86_64"
                metadata = "https://raw.githubusercontent.com/metis-os/hysp-pkgs/main/data/metadata.toml"
                aarch = "{}"
                [local]
                home = "{}/.local/share/hysp"
                bin = "{}/.local/share/hysp/bin/"
                data = "{}/.local/share/hysp/data/"
                "#,
                arch, home, home, home
            )
        }
    };

    let parsed_local_config = toml::from_str::<LocalConfig>(&contents)?;
    Ok(parsed_local_config)
}

/// Parse hysp packages config
#[inline]
pub async fn parse_pkg_info(pkg_url: &str) -> Result<PackageInfo> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10)) // Allow redirection limit = 10
        .danger_accept_invalid_certs(true) // allow http as well
        .build()?;

    let response = client.get(pkg_url).send().await?;
    let body = response.text().await?;
    let parsed_package_info: PackageInfo = toml::from_str(&body)?;
    Ok(parsed_package_info)
}

#[inline]
pub async fn parse_metadata_info(metadata_url: &str) -> Result<MetadataInfo> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10)) // Allow redirection limit = 10
        .danger_accept_invalid_certs(true) // allow http as well
        .build()?;

    let response = client.get(metadata_url).send().await?;
    let body = response.text().await?;
    let parsed_metadata_info: MetadataInfo = toml::from_str(&body)?;
    Ok(parsed_metadata_info)
}

#[cfg(test)]
mod tests {
    use crate::engine::helpers::{create_directory_if_not_exists, local_config};

    #[tokio::test]
    async fn test_directories_exist() {
        let (_hysp_remote, hysp_data_dir, hysp_bin_dir, _hysp_metadata, _architecture) =
            match local_config().await {
                Ok((remote, data_dir, bin_dir, metadata, architecture)) => {
                    (remote, data_dir, bin_dir, metadata, architecture)
                }
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1)
                }
            };

        create_directory_if_not_exists(&hysp_data_dir);
        create_directory_if_not_exists(&hysp_bin_dir);

        assert!(std::path::Path::new(&hysp_data_dir).exists());
        assert!(std::path::Path::new(&hysp_bin_dir).exists());
    }
}
