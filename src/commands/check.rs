use crate::engine::data_parser::fetch_and_process_toml;

pub async fn check_integrity(pkgname: &str) {
    if let Ok(toml_info) = fetch_and_process_toml(&pkgname).await {
        if let Some(hash) = toml_info.package.metadata.hash {
            println!("{:?}", hash);
        }
    } else {
        // Handle the case where fetching and processing the TOML failed
        println!("Failed to fetch and process TOML for {}", pkgname);
    }
}
