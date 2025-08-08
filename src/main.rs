use anyhow::Result;
use clap::Parser;
use std::process;

mod cli;
mod constants;
mod debug;
mod formatter;
mod modes;

use cli::Args;
use formatter::lineformatter::LineFormatter;

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

    let log_files = args.log_files.unwrap_or_else(|| {
        constants::DEFAULT_LOG_FILES
            .iter()
            .map(|s| s.to_string())
            .collect()
    });

    if args.debug {
        debug::print_debug_info(&log_files, formatter.get_include_regex(), formatter.get_exclude_regex());
    }

    if args.cat {
        modes::cat::run(log_files, formatter).await?;
    } else {
        modes::watch::run(log_files, formatter).await?;
    }

    Ok(())
}
