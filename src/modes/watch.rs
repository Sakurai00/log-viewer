use anyhow::{Context, Result};
use linemux::MuxedLines;

use crate::formatter::lineformatter::LineFormatter;
use crate::modes::output::{write_processed_line, NewlineMode};

pub async fn run(log_files: Vec<String>, formatter: LineFormatter) -> Result<()> {
    let mut log_reader = MuxedLines::new()?;
    for file in &log_files {
        log_reader
            .add_file(file)
            .await
            .with_context(|| format!("Failed to read file: {file}"))?;
    }

    while let Some(line) = log_reader
        .next_line()
        .await
        .with_context(|| format!("Failed while watching log files: {}", log_files.join(", ")))?
    {
        write_processed_line(line.line(), &formatter, NewlineMode::Append)?;
    }

    Ok(())
}
