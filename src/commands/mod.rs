pub mod health;
pub mod install;
pub mod list;
pub mod remove;
pub mod search;

pub mod hysp_cmd_helper {
    use anyhow::{Context, Result};
    use std::io::{self, Write};
    use tokio::{fs::File, io::AsyncReadExt};

    use crate::log::{abort, info};

    pub const BAR: &str = r"────────────────────────────────";

    pub const RESET: &str = "\x1B[0m"; // (resets the text color to the default)

    pub const ASCII: &str = "

*ੈ✩‧₊˚ *ੈ✩‧₊˚    
   __   _
 _(  )_( )_
(_   _    _)
  (_) (__)

*ੈ✩‧₊˚ *ੈ✩‧₊˚    
";

    /// Reads a file and returns the contents as string
    #[inline]
    pub async fn read_file_content(file_path: &str) -> Result<String> {
        let mut file = File::open(file_path)
            .await
            .with_context(|| format!("Failed to open file '{}'", file_path))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .with_context(|| format!("Failed to read file '{}'", file_path))?;

        Ok(contents)
    }
    #[inline]
    pub fn get_arch() -> Result<String, &'static str> {
        let architecture = std::env::consts::ARCH.to_string();

        match architecture.as_str() {
            "x86_64" | "aarch64" => Ok(architecture),
            _ => Err("Unsupported architecture"),
        }
    }

    #[inline]
    pub fn ask_to_continue() -> bool {
        loop {
            print!("Do you want to continue? (y/n): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let trimmed = input.trim().to_lowercase();
            match trimmed.as_str() {
                "yes" | "y" => {
                    info("Continuing ...", colored::Color::Green);
                    return true;
                }
                "no" | "n" => {
                    abort("Exitting.. ");
                }
                _ => {
                    println!("Please enter 'y|es' or 'n|o'.");
                }
            }
        }
    }
}
