use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

pub fn get_xdg_path(key: &str, default: &str) -> PathBuf {
    match env::var(key) {
        Ok(val) => PathBuf::from(val),
        Err(_) => {
            let xdg_home = env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
                format!("{}/.local/share", env::var("HOME").unwrap_or_default())
            });
            PathBuf::from(format!("{}/{}", xdg_home, default))
        }
    }
}

lazy_static! {
    pub static ref SEREN_HOME_DIR: PathBuf = get_xdg_path("SEREN_HOME_DIR", "seren/");
    pub static ref SEREN_BIN_DIR: PathBuf = get_xdg_path("SEREN_BIN_DIR", "seren/bin/");
    pub static ref SEREN_DATA_DIR: PathBuf = get_xdg_path("SEREN_DATA_DIR", "seren/data/");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    fn check_path(path: &std::path::PathBuf) -> Result<(), Error> {
        if path.exists() && path.is_absolute() {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                format!("Path does not exist or is not an absolute path: {:?}", path),
            ))
        }
    }

    #[test]
    fn test_seren_home_dir() {
        assert!(
            check_path(&SEREN_HOME_DIR).is_ok(),
            "SEREN_HOME_DIR does not exist or is not an absolute path"
        );
    }

    #[test]
    fn test_seren_bin_dir() {
        assert!(
            check_path(&SEREN_BIN_DIR).is_ok(),
            "SEREN_BIN_DIR does not exist or is not an absolute path"
        );
    }

    #[test]
    fn test_seren_data_dir() {
        assert!(
            check_path(&SEREN_DATA_DIR).is_ok(),
            "SEREN_DATA_DIR does not exist or is not an absolute path"
        );
    }
}
