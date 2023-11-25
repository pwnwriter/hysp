pub mod commands;
pub mod engine;
pub mod init;
pub mod log;

#[tokio::main]
async fn main() {
    init::start().await;
}
