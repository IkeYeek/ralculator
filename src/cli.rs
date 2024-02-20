use clap::Args;

/// Simple mathematical expression program
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub mode: Mode,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Mode {
    #[arg(short, long)]
    pub interactive: bool,
    #[arg(short, long)]
    pub exec: Option<String>,
}
