pub mod health;
pub mod install;
pub mod list;
pub mod remove;
pub mod search;

pub mod ui {

    pub const BAR: &str = r"────────────────────────────────────────────";

    pub const RESET: &str = "\x1B[0m"; // (resets the text color to the default)

    pub const ASCII: &str = "

*ੈ✩‧₊˚ *ੈ✩‧₊˚    
   __   _
 _(  )_( )_
(_   _    _)
  (_) (__)

*ੈ✩‧₊˚ *ੈ✩‧₊˚    
";
}
