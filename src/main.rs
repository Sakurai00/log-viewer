use anyhow::Result;
use clap::Parser;
use colored::{self, Colorize};
use linemux::MuxedLines;
use regex::Regex;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, value_parser, num_args=1..)]
    target_files: Option<Vec<String>>,
    #[arg(short = 'd', long = "dont-use-preset-exclude")]
    dont_use_preset_exclude: bool,
    #[arg(short, long, value_parser, num_args=1..)]
    exclude_string: Option<Vec<String>>,
    #[arg(short, long, value_parser, num_args=1..)]
    include_string: Option<Vec<String>>,
}

async fn set_target_file(input_target_files: Option<Vec<String>>) -> Result<MuxedLines> {
    let mut log_reader = MuxedLines::new()?;

    match input_target_files {
        Some(target_files) => {
            for file in target_files {
                log_reader.add_file(&file).await?;
            }
        }
        None => {
            log_reader.add_file("/var/log/messages").await?;
        }
    };

    Ok(log_reader)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut log_reader = set_target_file(args.target_files).await?;

    let default_exclude = ["aaa", "bbb", "ccc"];
    let default_exclude: Vec<String> = default_exclude.iter().map(|&s| s.to_string()).collect();

    let include_regex: Option<Regex> = match args.include_string {
        Some(x) => Some(Regex::new(&x.join("|"))?),
        None => None,
    };

    let exclude_regex: Option<Regex> = match args.exclude_string {
        Some(input) => {
            if args.dont_use_preset_exclude {
                Some(Regex::new(&input.join("|"))?)
            } else {
                let combined_exclude = vec![input, default_exclude]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<String>>()
                    .join("|");
                Some(Regex::new(&combined_exclude)?)
            }
        }
        None => {
            if args.dont_use_preset_exclude {
                None
            } else {
                Some(Regex::new(&default_exclude.join("|"))?)
            }
        }
    };

    println!("target: {:#?}", log_reader);
    println!("include: {:#?}", include_regex);
    println!("exclude: {:#?}", exclude_regex);

    while let Ok(Some(line)) = log_reader.next_line().await {
        let line = line.line();

        if let Some(ref e) = exclude_regex {
            if e.is_match(line) {
                continue;
            };
        };

        if let Some(ref e) = include_regex {
            if !e.is_match(line) {
                continue;
            };
        };
        color_println(line).await?;
    }

    Ok(())
}

pub async fn color_println(line: &str) -> Result<()> {
    let red_bold = ["foo", "bar"];
    let red_bold_regex = Regex::new(&red_bold.join("|"))?;
    let line = red_bold_regex.replace_all(line, |caps: &regex::Captures| {
        let matched = &caps[0];
        matched.bright_red().bold().to_string()
    });

    println!("{}", line);
    Ok(())
}
