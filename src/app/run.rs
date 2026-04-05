use anyhow::Result;
use clap::Parser;

use crate::cli::Args;
use crate::config::app_config::AppConfig;
use crate::core::processor::LineProcessor;
use crate::{app, debug};

pub async fn run() -> Result<()> {
    let args = Args::parse();
    let config = AppConfig::from(args);

    let processor = LineProcessor::new(
        config.include_words.clone(),
        config.exclude_words.clone(),
        config.disable_preset_excludes,
    )?;

    if config.debug {
        debug::print_debug_info(
            &config.log_files,
            processor.include_matcher(),
            processor.exclude_matcher(),
        );
    }

    if config.use_cat_mode {
        app::commands::cat::run(config.log_files, processor).await?;
    } else {
        app::commands::watch::run(config.log_files, processor).await?;
    }

    Ok(())
}
