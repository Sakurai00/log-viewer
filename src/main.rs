use anyhow::Result;
use clap::Parser;
use colored::{self, Colorize, ColoredString};
use linemux::MuxedLines;
use regex::Regex;
use std::borrow::Cow;

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

const DEFAULT_EXCLUDE: &[&str] = &["aaa", "bbb", "ccc"];

// Highlight keyword definitions
const KEYWORDS_CRITICAL: &[&str] = &["foo", "bar"];
const KEYWORDS_INFO: &[&str] = &["info", "success"];
const KEYWORDS_WARN: &[&str] = &["warning"];

enum Color {
    Red,
    BrightRed,
    Green,
    Yellow,
    Blue,
}

enum Style {
    Bold,
    Italic,
    Underline,
    Normal,
}

struct HighlightRule {
    keywords: Vec<String>,
    color: Color,
    style: Style,
}

fn get_highlight_rules() -> Vec<HighlightRule> {
    vec![
        HighlightRule {
            keywords: KEYWORDS_CRITICAL.iter().map(|&s| s.to_string()).collect(),
            color: Color::BrightRed,
            style: Style::Bold,
        },
        HighlightRule {
            keywords: KEYWORDS_INFO.iter().map(|&s| s.to_string()).collect(),
            color: Color::Green,
            style: Style::Normal,
        },
        HighlightRule {
            keywords: KEYWORDS_WARN.iter().map(|&s| s.to_string()).collect(),
            color: Color::Yellow,
            style: Style::Underline,
        },
    ]
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut log_reader = set_target_file(args.target_files).await?;
    let (include_regex, exclude_regex) = set_regex(
        args.include_targets,
        args.exclude_targets,
        args.dont_use_preset_exclude_targets,
    )?;
    let highlight_rules = get_highlight_rules();

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
        let highlighted_line = apply_highlighting(line, &highlight_rules)?;
        println!("{}", highlighted_line);
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

fn apply_highlighting<'a>(
    line: &'a str,
    rules: &[HighlightRule],
) -> Result<Cow<'a, str>> {
    let mut processed_line: Cow<str> = Cow::Borrowed(line);

    for rule in rules {
        let pattern = rule.keywords.join("|");
        let re = Regex::new(&pattern)?;

        if re.is_match(&processed_line) {
            processed_line = Cow::Owned(re.replace_all(&processed_line, |caps: &regex::Captures| {
                let matched = &caps[0];
                apply_style(matched, &rule.color, &rule.style).to_string()
            }).into_owned());
        }
    }

    Ok(processed_line)
}

fn apply_style(text: &str, color: &Color, style: &Style) -> ColoredString {
    let colored_text = match color {
        Color::Red => text.red(),
        Color::BrightRed => text.bright_red(),
        Color::Green => text.green(),
        Color::Yellow => text.yellow(),
        Color::Blue => text.blue(),
    };

    match style {
        Style::Bold => colored_text.bold(),
        Style::Italic => colored_text.italic(),
        Style::Underline => colored_text.underline(),
        Style::Normal => colored_text,
    }
}