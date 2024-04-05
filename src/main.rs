use clap::Parser;
use std::path::PathBuf;

pub mod cursor;
pub mod doc;
pub mod editor;
mod ropex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Args::parse();
    editor::Editor::react(&cli).await
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[warn(private_interfaces)]
struct Args {
    /// The path to the file to open
    file: Option<PathBuf>,
}
