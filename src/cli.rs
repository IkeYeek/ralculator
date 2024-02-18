use clap::Args;

/// Simple mathematical expression program
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) mode: Mode,
}

impl Cli {
    pub(crate) fn run_interactive() -> Result<(), String> {
        Ok(())
    }
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub(crate) struct Mode {
    #[arg(short, long)]
    pub(crate) interactive: bool,
    #[arg(short, long)]
    pub(crate) exec: Option<String>,
}
