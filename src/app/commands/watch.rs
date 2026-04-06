use anyhow::{Context, Result};
use linemux::MuxedLines;

use crate::app::commands::shared::{emit_processed_line, OutputMode};
use crate::core::processor::LineProcessor;

pub async fn run(log_files: Vec<String>, processor: LineProcessor) -> Result<()> {
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
        emit_processed_line(line.line(), &processor, OutputMode::Append)?;
    }

    Ok(())
}
