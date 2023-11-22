use std::env;
use std::fs;
use std::path::PathBuf;

lazy_static::lazy_static! {
    pub static ref HYSP_HOME_DIR: PathBuf = {
        let hysp_home = match env::var("HYSP_HOME_DIR") {
            Ok(val) => PathBuf::from(val),
            Err(_) => {
                let xdg_home = env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
                    format!("{}/.local/share", env::var("HOME").unwrap_or_default())
                });
                PathBuf::from(format!("{}/{}", xdg_home, "hysp"))
            }
        };
        create_if_not_exists(&hysp_home);
        hysp_home
    };
    pub static ref HYSP_BIN_DIR: PathBuf = {
        let bin_dir = HYSP_HOME_DIR.join("bin");
        create_if_not_exists(&bin_dir);
        bin_dir
    };
    pub static ref HYSP_DATA_DIR: PathBuf = {
        let data_dir = HYSP_HOME_DIR.join("data");
        create_if_not_exists(&data_dir);
        data_dir
    };
}

fn create_if_not_exists(dir: &PathBuf) {
    if let Err(_) = fs::create_dir_all(&dir) {
        println!("Failed to create directory: {:?}", dir);
    }
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
