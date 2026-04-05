use anyhow::{Context, Result};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::app::commands::shared::{emit_processed_line, OutputMode};
use crate::core::processor::LineProcessor;

pub async fn run(log_files: Vec<String>, processor: LineProcessor) -> Result<()> {
    for file_path in log_files {
        let file = File::open(&file_path)
            .await
            .with_context(|| format!("Failed to open file: {file_path}"))?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        while reader.read_line(&mut line).await? > 0 {
            emit_processed_line(&line, &processor, OutputMode::PreserveExisting)?;
            line.clear();
        }
    }
    Ok(())
}
