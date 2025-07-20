use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, value_parser, num_args=1..)]
    pub log_files: Option<Vec<String>>,
    #[arg(short = 'd', long = "disable-preset-excludes")]
    pub disable_preset_excludes: bool,
    #[arg(short, long, value_parser, num_args=1..)]
    pub exclude_words: Option<Vec<String>>,
    #[arg(short, long, value_parser, num_args=1..)]
    pub include_words: Option<Vec<String>>,
    #[arg(long)]
    pub debug: bool,
    #[arg(long)]
    pub cat: bool,
}
