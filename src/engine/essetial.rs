use anyhow::Result;
use sha2::{Digest, Sha256};
use spinoff::{spinners, Color, Spinner, Streams};
use std::process::Command;
use std::{
    fs::File,
    io::{self, Read},
};

pub async fn check_hash(pkgname: String, expected_hash: String) -> Result<bool, io::Error> {
    let mut spinner_hash = Spinner::new_with_stream(
        spinners::Line,
        "Validating hash ... ",
        Color::Green,
        Streams::Stderr,
    );

    let mut file = File::open(pkgname)?;

    let mut hasher = Sha256::new();

    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let actual_hash = format!("{:x}", hasher.finalize());
    spinner_hash.stop_and_persist(" Validating hash  ", "Done");

    Ok(actual_hash == expected_hash)
}

pub fn is_pkg_installed(pkg_name: &str) -> bool {
    let output = Command::new("which")
        .arg(pkg_name)
        .output()
        .expect("Failed to execute 'which' command");

    output.status.success()
}
