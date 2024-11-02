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
    dont_use_preset_exclude_targets: bool,
    #[arg(short, long, value_parser, num_args=1..)]
    exclude_targets: Option<Vec<String>>,
    #[arg(short, long, value_parser, num_args=1..)]
    include_targets: Option<Vec<String>>,
}

const DEFAULT_EXCLUDE: [&str; 3] = ["aaa", "bbb", "ccc"];
const RED_BOLD: [&str; 2] = ["foo", "bar"];


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut log_reader = set_target_file(args.target_files).await?;
    let (include_regex, exclude_regex) = set_regex(
        args.include_targets,
        args.exclude_targets,
        args.dont_use_preset_exclude_targets,
    )?;

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

fn set_regex(
    input_include_targets: Option<Vec<String>>,
    input_exclude_targets: Option<Vec<String>>,
    dont_use_preset_exclude_targets: bool,
) -> Result<(Option<Regex>, Option<Regex>)> {
    let default_exclude: Vec<String> = DEFAULT_EXCLUDE.iter().map(|&s| s.to_string()).collect();
    let use_preset = !dont_use_preset_exclude_targets;

    let include_regex: Option<Regex> = match input_include_targets {
        Some(input) => Some(Regex::new(&input.join("|"))?),
        None => None,
    };

    let exclude_regex: Option<Regex> = match input_exclude_targets {
        Some(input) => {
            if use_preset {
                let combined_exclude = vec![input, default_exclude]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<String>>()
                    .join("|");
                Some(Regex::new(&combined_exclude)?)
            } else {
                Some(Regex::new(&input.join("|"))?)
            }
        }
        None => {
            if use_preset {
                Some(Regex::new(&default_exclude.join("|"))?)
            } else {
                None
            }
        }
    };

    Ok((include_regex, exclude_regex))
}

async fn color_println(line: &str) -> Result<()> {
    let red_bold_regex = Regex::new(&RED_BOLD.join("|"))?;
    let line = red_bold_regex.replace_all(line, |caps: &regex::Captures| {
        let matched = &caps[0];
        matched.bright_red().bold().to_string()
    });

    println!("{}", line);
    Ok(())
}
