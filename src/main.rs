use anyhow::{Context, Result};
use clap::Parser;
use linemux::MuxedLines;
use std::process;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

mod cli;
mod debug;
mod filtering;
mod formatter;
mod highlighting;

use cli::Args;
use formatter::LineFormatter;

const PRESET_EXCLUDE_WORDS: &[&str] = &["aaa", "bbb", "ccc"];
const DEFAULT_LOG_FILES: &[&str] = &["/var/log/messages"];

// Highlight keyword definitions
const CRITICAL_WORDS: &[&str] = &["foo", "bar"];
const INFO_WORDS: &[&str] = &["info", "success"];
const WARN_WORDS: &[&str] = &["warning"];

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e:?}");
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();

    let formatter = LineFormatter::new(args.include_words, args.exclude_words, args.disable_preset_excludes)?;

    let log_files = args
        .log_files
        .unwrap_or_else(|| DEFAULT_LOG_FILES.iter().map(|s| s.to_string()).collect());

    if args.debug {
        debug::print_debug_info(&log_files, formatter.get_include_regex(), formatter.get_exclude_regex());
    }

    if args.cat {
        run_cat(log_files, formatter).await?;
    } else {
        run_watch(log_files, formatter).await?;
    }

    Ok(())
}

async fn run_watch(log_files: Vec<String>, formatter: LineFormatter) -> Result<()> {
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

async fn run_cat(log_files: Vec<String>, formatter: LineFormatter) -> Result<()> {
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
