use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use indicatif::MultiProgress;
#[cfg(feature = "progress")]
use indicatif::{ProgressBar, ProgressStyle};
use tokio::task::JoinSet;

use crate::cli::Command;
use crate::error::Error;
use crate::timestamp::TimestampRange;

pub mod cli;
pub mod error;
pub mod timestamp;

#[cfg(feature = "progress")]
const PROGRESS_STYLE_TEMPLATE: &str =
    "{prefix:<30} | {elapsed_precise} {bar:<30} {percent:>3}% [{pos}/{len}] [ETA: {eta}]";

pub async fn delay_srt(command: &Command) -> Vec<Result<(), Error>> {
    let mut join_set = JoinSet::new();

    #[cfg(feature = "progress")]
    let multi_progress = MultiProgress::new();
    #[cfg(feature = "progress")]
    let progress_style = ProgressStyle::with_template(PROGRESS_STYLE_TEMPLATE).unwrap();

    for input_file in command.input_files.iter().cloned() {
        let delay_ms = command.delay_ms;
        let output_directory = command.output_directory.clone();

        #[cfg(feature = "progress")]
        let multi_progress = multi_progress.clone();
        #[cfg(feature = "progress")]
        let progress_style = progress_style.clone();

        join_set.spawn(async move {
            if input_file.extension() != Some(OsStr::new("srt")) {
                return Err(Error::InvalidFile(input_file.clone()));
            }

            // File name must exist if extension is .srt
            let input_file_name = input_file.file_name().unwrap();

            let content = fs::read_to_string(&input_file)?;
            let mut new_content = String::with_capacity(content.len());

            #[cfg(feature = "progress")]
            let progress: ProgressBar = {
                let line_count = content.lines().count() as u64;
                let progress = ProgressBar::new(line_count)
                    .with_style(progress_style.clone())
                    .with_prefix(input_file.to_string_lossy().to_string());

                multi_progress.add(progress)
            };

            for line in content.lines() {
                match TimestampRange::from_str(line) {
                    Ok(timestamp_range) => {
                        // Update the timestamps according to the delay
                        new_content.push_str(&timestamp_range.delay(delay_ms).as_string());
                    }
                    Err(_) => {
                        // Leave the line unchanged
                        new_content.push_str(line);
                    }
                }

                new_content.push('\n');

                #[cfg(feature = "progress")]
                progress.inc(1);
            }

            let output_file = output_directory.join(PathBuf::from(input_file_name));
            fs::write(output_file, new_content)?;

            Ok(())
        });
    }

    join_set.join_all().await
}
