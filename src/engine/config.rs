use crate::commands::hysp_cmd_helper::read_file_content;
use crate::commands::hysp_cmd_helper::BAR;

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

use crate::commands::hysp_cmd_helper::get_arch;
use crate::log::info;
use anyhow::{anyhow, Result};
use colored::Colorize;
use local_config_structure::LocalConfig;

pub async fn read_config() -> Result<LocalConfig> {
    let config_file_path = dirs::config_dir()
        .ok_or_else(|| anyhow!("Config directory not found"))?
        .join("hysp/config.toml");

    let default_arch = get_arch().unwrap_or_else(|err| {
        eprintln!("Error getting architecture: {}", err);
        std::process::exit(1);
    });

    let contents = match read_file_content(&config_file_path.to_string_lossy()).await {
        Ok(contents) => contents,
        Err(_) => {
            info("Using default configuration.", colored::Color::Green);
            println!("{}", BAR.bold().purple());
            let home = match std::env::var("HOME") {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("Error: HOME environment variable not found");
                    std::process::exit(0);
                }
            };
            let default_config_toml = format!(
                r#"
        [source]
        remote = "https://raw.githubusercontent.com/metis-os/hysp-pkgs/main/data/"
        aarch = "{}"
        [local]
        home = "{}/.local/share/hysp"
        bin = "{}/.local/share/hysp/bin/"
        data = "{}/.local/share/hysp/data/x86_64/"
        "#,
                default_arch, home, home, home
            );
            default_config_toml.to_string()
        }
    };

    let parsed_config_toml = toml::from_str::<LocalConfig>(&contents)?;
    Ok(parsed_config_toml)
}
