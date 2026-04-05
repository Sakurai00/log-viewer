use anyhow::{Context, Result};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::formatter::lineformatter::LineFormatter;
use crate::modes::output::{write_processed_line, NewlineMode};

pub async fn run(log_files: Vec<String>, formatter: LineFormatter) -> Result<()> {
    for file_path in log_files {
        let file = File::open(&file_path)
            .await
            .with_context(|| format!("Failed to open file: {file_path}"))?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        while reader.read_line(&mut line).await? > 0 {
            write_processed_line(&line, &formatter, NewlineMode::PreserveExisting)?;
            line.clear();
        }
    }
    Ok(())
}
