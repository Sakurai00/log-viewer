use clap::Parser;

use crate::constants::DEFAULT_LOG_FILES;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(value_name = "LOG_FILES", num_args = 1.., default_values = DEFAULT_LOG_FILES)]
    pub log_files: Vec<String>,
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

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::Parser;

    #[test]
    fn parses_default_log_file_when_none_is_provided() {
        let args = Args::parse_from(["log-viewer"]);

        assert_eq!(args.log_files, vec!["/var/log/messages"]);
    }

    #[test]
    fn parses_positional_log_files() {
        let args = Args::parse_from(["log-viewer", "--cat", "/tmp/app.log", "/tmp/worker.log"]);

        assert!(args.cat);
        assert_eq!(args.log_files, vec!["/tmp/app.log", "/tmp/worker.log"]);
    }
}
