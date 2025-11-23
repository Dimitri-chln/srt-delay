use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Command {
    pub delay_ms: i64,
    pub input_files: Vec<PathBuf>,
    pub output_directory: PathBuf,
}
