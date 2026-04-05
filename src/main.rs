use anyhow::Result;
use std::process;

mod app;
mod cli;
mod config;
mod constants;
mod core;
mod debug;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e:?}");
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    app::run::run().await
}
