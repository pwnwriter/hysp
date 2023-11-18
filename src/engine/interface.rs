// LICENSE: https://en.wikipedia.org/wiki/Creative_Commons_license

use colored::Colorize;

pub static SPLASHES: &[&str] = &[
    "There are reasons to use rust. - PwnWriter",
    "whatsoever a man soweth, that shall he also reap. - Dylanaraps",
];

fn generate_random_number() -> usize {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros();

    (current_time % SPLASHES.len() as u128) as usize
}

pub fn show_splashes() -> String {
    let rng = generate_random_number();

    let app_version = env!("CARGO_PKG_VERSION");

    let logo = format!(
        r#"

 ✮⋆˙┏┓┏┓┳┓┏┓┳┓ ✮⋆˙
    ┗┓┣ ┣┫┣ ┃┃
    ┗┛┗┛┛┗┗┛┛┗ v{}
              
    "#,
        app_version,
    )
    .bold()
    .purple();
    let splash = SPLASHES[rng].italic();
    format!("{logo} {splash}")
}
