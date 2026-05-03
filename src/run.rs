use anyhow::{Context, Result};
use linemux::MuxedLines;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::line_pipeline::LinePipeline;

enum OutputMode {
    PreserveExistingLineEnding,
    AppendLineEnding,
}

pub async fn run_cat(log_files: Vec<String>, pipeline: LinePipeline) -> Result<()> {
    for file_path in log_files {
        let file = File::open(&file_path)
            .await
            .with_context(|| format!("Failed to open file: {file_path}"))?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        while reader.read_line(&mut line).await? > 0 {
            emit_processed_line(&line, &pipeline, OutputMode::PreserveExistingLineEnding)?;
            line.clear();
        }
    }

    Ok(())
}

pub async fn run_watch(log_files: Vec<String>, pipeline: LinePipeline) -> Result<()> {
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
        emit_processed_line(line.line(), &pipeline, OutputMode::AppendLineEnding)?;
    }

    Ok(())
}

fn emit_processed_line(line: &str, pipeline: &LinePipeline, output_mode: OutputMode) -> Result<()> {
    if let Some(processed_line) = pipeline.process(line) {
        match output_mode {
            OutputMode::PreserveExistingLineEnding => print!("{processed_line}"),
            OutputMode::AppendLineEnding => println!("{processed_line}"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{emit_processed_line, OutputMode};
    use crate::line_pipeline::LinePipeline;

    #[test]
    fn processed_line_can_be_written_without_error() {
        let pipeline = LinePipeline::new(None, None, true).unwrap();

        assert!(emit_processed_line("plain log line", &pipeline, OutputMode::AppendLineEnding).is_ok());
        assert!(emit_processed_line("plain log line\n", &pipeline, OutputMode::PreserveExistingLineEnding).is_ok());
    }
}
