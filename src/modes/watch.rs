use anyhow::{Context, Result};
use linemux::MuxedLines;

use crate::formatter::lineformatter::LineFormatter;

pub async fn run(log_files: Vec<String>, formatter: LineFormatter) -> Result<()> {
    let mut log_reader = MuxedLines::new()?;
    for file in &log_files {
        log_reader
            .add_file(file)
            .await
            .with_context(|| format!("Failed to read file: {file}"))?;
    }

    while let Ok(Some(line)) = log_reader.next_line().await {
        if let Some(processed_line) = formatter.process_line(line.line()) {
            println!("{processed_line}");
        }
    }
    Ok(())
}
