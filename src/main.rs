use std::process::ExitCode;

use clap::Parser;
use srt_delay::cli::Command;
use srt_delay::delay_srt;

#[tokio::main]
async fn main() -> ExitCode {
    let command = Command::parse();
    let results = delay_srt(&command).await;

    for (input_file, result) in command.input_files.iter().zip(results) {
        if let Err(error) = result {
            eprintln!(
                "An error occurred while processing file {}: {}",
                input_file.display(),
                error
            );
        }
    }

    return ExitCode::SUCCESS;
}
