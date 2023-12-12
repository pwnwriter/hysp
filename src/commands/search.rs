use crate::{
    commands::ui::BAR,
    engine::{
        args::{Modes, SearchArgs},
        config::{parse_metadata_info, parse_pkg_info},
        helpers::{
            local_config, print_metadata_package_info, print_package_info, remove_trailing_slash,
        },
        msgx::info,
    },
};
use anyhow::Result;
use colored::Colorize;
use reqwest::StatusCode;
use spinoff::{spinners, Color, Spinner, Streams};

pub async fn search_pkgs(pkg_search_args: SearchArgs) -> Result<()> {
    let (hysp_remote, _hysp_data_dir, _hysp_bin_dir, hysp_metadata) = match local_config().await {
        Ok((remote, data_dir, bin_dir, metadata)) => (remote, data_dir, bin_dir, metadata),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let package_name = pkg_search_args.package;
    let limit = pkg_search_args.limit;

    match pkg_search_args.mode {
        Modes::database => {
            print_info_using_metadata(&package_name, &hysp_metadata).await?;
        }
        Modes::raw => {
            print_info_using_raw(&package_name, &hysp_remote).await?;
        }
        Modes::fuzzy => {
            search_similar_packages(&hysp_metadata, &package_name, limit).await?;
        }
    }

    Ok(())
}

async fn print_info_using_metadata(package_name: &str, hysp_metadata: &str) -> Result<()> {
    let mut spinner_search_pkgs = Spinner::new_with_stream(
        spinners::Dots,
        "Using metadata to check for packages .. ",
        Color::Cyan,
        Streams::Stderr,
    );

    let available_pkg_info = parse_metadata_info(hysp_metadata).await?;
    spinner_search_pkgs.stop_and_persist("Using metadata to check for packages  ", "Done ");
    print_metadata_package_info(available_pkg_info, package_name);

    Ok(())
}

async fn print_info_using_raw(package_name: &str, hysp_remote: &str) -> Result<(), anyhow::Error> {
    info(
        "Using raw method to check for packages",
        colored::Color::BrightBlue,
    );
    let package_url = format!(
        "{}/{}.toml",
        remove_trailing_slash(hysp_remote.to_string()),
        package_name
    );

    let response = reqwest::get(&package_url).await?;

    match response.status() {
        StatusCode::OK => {
            info(
                &format!("Package found as: {}", package_name.bold()),
                colored::Color::Cyan,
            );

            let parsed_pkg_info = parse_pkg_info(&package_url).await?;
            print_package_info(parsed_pkg_info);
            Ok(())
        }
        StatusCode::NOT_FOUND => {
            info(
                &format!("No such package found as: {}", package_name),
                colored::Color::Cyan,
            );
            Ok(())
        }
        _ => Err(anyhow::anyhow!(
            "Unexpected status code: {}",
            response.status()
        )),
    }
}

async fn search_similar_packages(
    hysp_metadata: &str,
    package_name: &str,
    package_limit: u16,
) -> Result<(), anyhow::Error> {
    let mut spinner_search_similar_pkgs = Spinner::new_with_stream(
        spinners::Dots,
        "Searching for databases in metadata ... ",
        Color::Green,
        Streams::Stderr,
    );
    let available_pkg_info = parse_metadata_info(hysp_metadata).await?;

    spinner_search_similar_pkgs.stop_and_persist("Searching for package in metadata   ", "Done ");

    let mut printed_packages = 0;
    let mut found_packages = false;

    for package in available_pkg_info.packages {
        if printed_packages >= package_limit {
            break;
        }

        let lowercase_package_name = package_name.to_lowercase();
        let lowercase_package = package.name.to_lowercase();
        let lowercase_description = package.description.to_lowercase();

        if lowercase_package.contains(&lowercase_package_name)
            || lowercase_description.contains(&lowercase_package_name)
        {
            found_packages = true; // At least one package found

            let highlighted_name = package
                .name
                .replacen(&lowercase_package_name, &package_name.cyan().to_string(), 1)
                .replacen(
                    &lowercase_package_name.to_uppercase(),
                    &package_name.cyan().to_uppercase().to_string(),
                    1,
                );

            let highlighted_desc = package
                .description
                .replacen(&lowercase_package_name, &package_name.cyan().to_string(), 1)
                .replacen(
                    &lowercase_package_name.to_uppercase(),
                    &package_name.cyan().to_uppercase().to_string(),
                    1,
                );

            println!("Package: {}", highlighted_name);
            println!("Description: {}", highlighted_desc);
            println!("{}", BAR);

            printed_packages += 1;
        }
    }

    if !found_packages {
        info("No such package exist in metadata", colored::Color::Red);
    }
    Ok(())
}
