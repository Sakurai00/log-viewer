use anyhow::{Context, Result};
use clap::Parser;
use linemux::MuxedLines;
use std::process;

mod cli;
mod debug;
mod filtering;
mod highlighting;

use cli::Args;
use filtering::{build_exclude_regex, build_include_regex, should_display_line};
use highlighting::{apply_highlighting, get_highlight_rules};

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

    let mut log_reader = get_log_reader(&args.log_files).await?;

    let include_regex = build_include_regex(args.include_words)?;
    let exclude_regex = build_exclude_regex(args.exclude_words, args.disable_preset_excludes)?;
    let highlight_rules = get_highlight_rules()?;

    if args.debug {
        debug::print_debug_info(&args.log_files, &include_regex, &exclude_regex, DEFAULT_LOG_FILES);
    }

    while let Ok(Some(line)) = log_reader.next_line().await {
        let line = line.line();

        if should_display_line(line, &include_regex, &exclude_regex) {
            let highlighted_line = apply_highlighting(line, &highlight_rules);
            println!("{highlighted_line}");
        }
    }

    Ok(())
}

async fn get_log_reader(log_files: &Option<Vec<String>>) -> Result<MuxedLines> {
    let mut log_reader = MuxedLines::new()?;

    let log_files = match log_files {
        Some(log_files) => log_files.clone(),
        None => DEFAULT_LOG_FILES.iter().map(|s| s.to_string()).collect(),
    };

    for file in &log_files {
        log_reader
            .add_file(file)
            .await
            .with_context(|| format!("Failed to read file: {file}"))?;
    }

    Ok(log_reader)
}
