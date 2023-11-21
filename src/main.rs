pub mod commands;
pub mod engine;
pub mod log;
pub mod init;

#[tokio::main]
async fn main() {
    init::start().await;
}
