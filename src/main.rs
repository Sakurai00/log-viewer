use anyhow::Result;
use std::process;

mod cli;
mod config;
mod constants;
mod debug;
mod line_highlighter;
mod line_filter;
mod line_pipeline;
mod run;
mod word_pattern;

use clap::Parser;

use crate::cli::Args;
use crate::config::AppConfig;
use crate::line_pipeline::LinePipeline;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e:?}");
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();
    let config = AppConfig::from(args);

    let pipeline = LinePipeline::new(
        config.include_words.clone(),
        config.exclude_words.clone(),
        config.disable_preset_excludes,
    )?;

    if config.debug {
        debug::print_debug_info(&config.log_files, pipeline.include_regex(), pipeline.exclude_regex());
    }

    if config.use_cat_mode {
        run::run_cat(config.log_files, pipeline).await?;
    } else {
        run::run_watch(config.log_files, pipeline).await?;
    }

    Ok(())
}
