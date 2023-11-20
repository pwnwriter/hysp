use crate::engine::data_parser::fetch_package_info;
use crate::engine::InstallArgs;
use std::fs::File;
use std::io::copy;
use tempfile::tempdir;

pub async fn download_pkgs(install_pkgs: InstallArgs) -> Result<(), anyhow::Error> {
    let pkgname = &install_pkgs.package;

    let toml_info = fetch_package_info(pkgname).await?;
    let binary_url = toml_info.source.url;

    download_binary(binary_url).await
}

async fn download_binary(binary_url: String) -> Result<(), anyhow::Error> {
    let response = reqwest::get(&binary_url).await?;
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let tmp_dir = tempdir()?;
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };

    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
