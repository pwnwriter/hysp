pub mod install;
pub mod list;
pub mod search;
pub mod uninstall;

pub mod hysp_helpers {

    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    pub const BAR: &str = r"────────────────────────────────";

    pub const RESET: &str = "\x1B[0m"; // (resets the text color to the default)

    pub const ASCII: &str = "
   __   _
 _(  )_( )_
(_   _    _)
  (_) (__)";

    /// Reads a file
    #[inline]

    pub async fn read_local_file(file_path: &str) -> Result<String, std::io::Error> {
        let mut file = File::open(file_path).await?;
        let mut contents = String::new();

        // Read the entire file asynchronously into a String
        file.read_to_string(&mut contents).await?;
        Ok(contents)
    }
}
