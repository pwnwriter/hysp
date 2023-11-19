use clap::Args;

#[derive(Args, Clone)]
pub struct InstallArgs {
    /// Name of the package to install
    #[arg(short, long)]
    #[arg(required = true)]
    pub package: String,

    #[arg(short, long)]
    #[arg(required = true)]
    pub verbose: bool,
}

#[derive(Args, Clone)]
pub struct RemoveArgs {
    /// Name of the package to Uninstall
    #[arg(short, long)]
    #[arg(required = true)]
    pub package: String,
}
