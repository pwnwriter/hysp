use crate::engine::{args::QueryArgs, msgx::abort};
use anyhow::Result;

pub async fn query_info(pkg_query_args: QueryArgs) -> Result<()> {
    match (
        pkg_query_args.print_default_config,
        pkg_query_args.print_example_package,
    ) {
        (true, false) => {
            print_default_config();
        }
        (false, true) => {
            print_example_pkg_toml();
        }
        _ => {
            abort("No such arg available");
        }
    }

    Ok(())
}

fn print_example_pkg_toml() {
    let example_toml = " 
[bin]
name = \"$BIN\" # Name of the pkg to be installed as

[package]
architecture = \"x86_64\" # Your architecture 
name = \"$BIN\" # Your package name
description = \"$DESCRIPTION\" # Description
author = \"$AUTHOR\" # Author 
repo = \"$REPO_URL\" 
stars = \"${STARS}\"
version = \"$PKG_VERSION\"
updated = \"$PKG_RELEASED\"
size = \"$SIZE\"
sha = \"$SHA\" 
source = \"$SOURCE_URL\" # Source of the binary wherever it's hosted
language = \"$LANGUAGE\"
license = \"$LICENSE\"

[package.conditions]
conflicts  = [\"$BIN\"] # Conflicts 
requires = [] # Dependencies 

[package.metadata]
keywords = $TOPICS
categories = [\"Utilities\"]
";

    println!("{}", example_toml);
}
fn print_default_config() {
    let def_toml = "
[source]
remote = \"https://raw.githubusercontent.com/metis-os/hysp-pkgs/main/data/x86_64\"
metadata =\"https://raw.githubusercontent.com/metis-os/hysp-pkgs/main/data/metadata.toml\"
aarch = \"Architecture\"

[local]   
home=\"/home/user/.local/share/hysp\"
bin=\"/home/user/.local/share/hysp/bin/\" 
data=\"/home/user/.local/share/hysp/data/\" 
";

    println!("{}", def_toml);
}
