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
    pub static ref HYSP_HOME_DIR: PathBuf = get_xdg_path("HYSP_HOME_DIR", "hysp");
    pub static ref HYSP_BIN_DIR: PathBuf = get_xdg_path(" HYSP_BIN_DIR", "hysp/bin");
    pub static ref HYSP_DATA_DIR: PathBuf = get_xdg_path("HYSP_DATA_DIR", "hysp/data");
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
    fn test_hysp_home_dir() {
        assert!(
            check_path(&HYSP_HOME_DIR).is_ok(),
            "HYSP_HOME_DIR does not exist or is not an absolute path"
        );
    }

    #[test]
    fn test_hysp_bin_dir() {
        assert!(
            check_path(&HYSP_BIN_DIR).is_ok(),
            "HYSP_BIN_DIR does not exist or is not an absolute path"
        );
    }

    #[test]
    fn test_hysp_data_dir() {
        assert!(
            check_path(&HYSP_DATA_DIR).is_ok(),
            "HYSP_DATA_DIR does not exist or is not an absolute path"
        );
    }
}
