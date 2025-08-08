use anyhow::{Context, Result};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::formatter::lineformatter::LineFormatter;

pub async fn run(log_files: Vec<String>, formatter: LineFormatter) -> Result<()> {
    for file_path in log_files {
        let file = File::open(&file_path)
            .await
            .with_context(|| format!("Failed to open file: {file_path}"))?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        while reader.read_line(&mut line).await? > 0 {
            if let Some(processed_line) = formatter.process_line(&line) {
                print!("{processed_line}");
            }
            line.clear();
        }
    }
    Ok(())
}
