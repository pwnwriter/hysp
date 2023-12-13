use crate::engine::helpers::remove_and_print;
use crate::engine::{
    args::RemoveArgs,
    config::pkg_config_structure::PackageInfo,
    helpers::{
        local_config, print_package_info, prompt_yn, read_file_content, remove_trailing_slash,
    },
    msgx::info,
};
use anyhow::Result;

pub async fn remove_pkgs(pkg_remove_args: RemoveArgs) -> Result<()> {
    for package in pkg_remove_args.packages {
        remove_package(&package, pkg_remove_args.force, pkg_remove_args.quiet).await?;
    }
    Ok(())
}

pub async fn remove_package(package_name: &str, force: bool, quiet: bool) -> Result<()> {
    let (_hysp_remote, hysp_data_dir, hysp_bin_dir, _hysp_metadata, _architecture) =
        match local_config().await {
            Ok((remote, data_dir, bin_dir, metadata, architecture)) => {
                (remote, data_dir, bin_dir, metadata, architecture)
            }
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

    let package_toml_path = format!(
        "{}/{}.toml",
        remove_trailing_slash(hysp_data_dir.clone()),
        package_name
    );

    let package_toml = read_file_content(&package_toml_path).await;
    match package_toml {
        Ok(package_toml) => {
            let parsed_package_info: PackageInfo = toml::from_str(&package_toml)?;

            if !quiet {
                print_package_info(parsed_package_info.clone());
            }

            let package_binary_name = parsed_package_info.bin.name;
            let package_binary_path = format!(
                "{}/{}",
                remove_trailing_slash(hysp_bin_dir.clone()),
                package_binary_name
            );

            if !force {
                info(
                    &format!("Removing package : {} ", package_binary_name),
                    colored::Color::Cyan,
                );
                if !prompt_yn("Would you like to proceed with the transaction (y/n)? ".to_string())
                {
                    return Ok(());
                }
            }

            let dependencies: Vec<String> = parsed_package_info.package.conditions.requires;
            let dependencies_str: String = dependencies.join(", ");

            if !dependencies.is_empty() {
                info(
                    &format!("Dependencies detected, removing : {} ", dependencies_str),
                    colored::Color::Cyan,
                );
            }

            for dependency in dependencies {
                let dependent_toml_path = format!(
                    "{}/{}.toml",
                    remove_trailing_slash(hysp_data_dir.clone()),
                    dependency
                );

                let dependent_package_toml = read_file_content(&dependent_toml_path).await?;

                let parsed_dependent_package_info: PackageInfo =
                    toml::from_str(&dependent_package_toml)?;

                let dependent_binary_path = format!(
                    "{}/{}",
                    remove_trailing_slash(hysp_bin_dir.clone()),
                    parsed_dependent_package_info.bin.name
                );
                remove_and_print(&dependent_binary_path);
                remove_and_print(&dependent_toml_path);
            }

            remove_and_print(&package_binary_path);
            remove_and_print(&package_toml_path);

            Ok(())
        }
        Err(_e) => {
            info(
                &format!("No such package found as : {} ", package_toml_path),
                colored::Color::Cyan,
            );
            Ok(())
        }
    }
}
