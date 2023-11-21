pub mod install;
pub mod search;
pub mod uninstall;

pub mod seren_helpers {

    pub const BAR: &str = r"
────────────────────────────────
";

    pub const RESET: &str = "\x1B[0m"; // (resets the text color to the default)
}
