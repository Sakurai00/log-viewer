use anyhow::Result;
use clap::Parser;
use colored::{self, Colorize, ColoredString};
use linemux::MuxedLines;
use regex::Regex;
use std::borrow::Cow;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, value_parser, num_args=1..)]
    log_files: Option<Vec<String>>,
    #[arg(short = 'd', long = "disable-preset-excludes")]
    disable_preset_excludes: bool,
    #[arg(short, long, value_parser, num_args=1..)]
    exclude_patterns: Option<Vec<String>>,
    #[arg(short, long, value_parser, num_args=1..)]
    include_words: Option<Vec<String>>,
    #[arg(long)]
    debug: bool,
}

const PRESET_EXCLUDE_PATTERNS: &[&str] = &["aaa", "bbb", "ccc"];

// Highlight keyword definitions
const CRITICAL_WORDS: &[&str] = &["foo", "bar"];
const INFO_WORDS: &[&str] = &["info", "success"];
const WARN_WORDS: &[&str] = &["warning"];

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
    regex: Regex,
    color: Color,
    style: Style,
}

fn get_highlight_rules() -> Result<Vec<HighlightRule>> {
    Ok(vec![
        HighlightRule {
            regex: Regex::new(&CRITICAL_WORDS.join("|"))?,
            color: Color::BrightRed,
            style: Style::Bold,
        },
        HighlightRule {
            regex: Regex::new(&INFO_WORDS.join("|"))?,
            color: Color::Green,
            style: Style::Normal,
        },
        HighlightRule {
            regex: Regex::new(&WARN_WORDS.join("|"))?,
            color: Color::Yellow,
            style: Style::Underline,
        },
    ])
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut log_reader = set_target_file(args.log_files).await?;
    let (include_regex, exclude_regex) = set_regex(
        args.include_patterns,
        args.exclude_patterns,
        args.disable_preset_excludes,
    )?;
    let highlight_rules = get_highlight_rules()?;

    if args.debug {
        print_debug_info(&args.log_files, &include_regex, &exclude_regex);
    }

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
        let highlighted_line = apply_highlighting(line, &highlight_rules);
        println!("{}", highlighted_line);
    }

    Ok(())
}

async fn set_target_file(log_files: Option<Vec<String>>) -> Result<MuxedLines> {
    let mut log_reader = MuxedLines::new()?;

    match log_files {
        Some(log_files) => {
            for file in log_files {
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
    include_patterns: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
    disable_preset_excludes: bool,
) -> Result<(Option<Regex>, Option<Regex>)> {
    let default_exclude_patterns: Vec<String> = PRESET_EXCLUDE_PATTERNS.iter().map(|&s| s.to_string()).collect();
    let use_preset_excludes = !disable_preset_excludes;

    let include_regex: Option<Regex> = match include_patterns {
        Some(patterns) => Some(Regex::new(&patterns.join("|"))?),
        None => None,
    };

    let exclude_regex: Option<Regex> = match exclude_patterns {
        Some(patterns) => {
            if use_preset_excludes {
                let combined_pattern_str = vec![patterns, default_exclude_patterns]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<String>>()
                    .join("|");
                Some(Regex::new(&combined_pattern_str)?)
            } else {
                Some(Regex::new(&patterns.join("|"))?)
            }
        }
        None => {
            if use_preset_excludes {
                Some(Regex::new(&default_exclude_patterns.join("|"))?)
            } else {
                None
            }
        }
    };

    Ok((include_regex, exclude_regex))
}

fn print_debug_info(
    log_files: &Option<Vec<String>>,
    include_regex: &Option<Regex>,
    exclude_regex: &Option<Regex>,
) {
    println!();
    println!("{}", "=".repeat(40).cyan());
    println!("{}", "  DEBUG INFO".bold().cyan());
    println!("{}", "=".repeat(40).cyan());

    // Log files
    match log_files {
        Some(files) => println!(
            "{}: {}",
            "Log files".bold(),
            files.join(", ")
        ),
        None => println!("{}: {}", "Log files".bold(), "/var/log/messages"),
    }

    // Include regex
    match include_regex {
        Some(regex) => println!("{}: {}", "Include Regex".bold(), regex.to_string()),
        None => println!("{}: {}", "Include Regex".bold(), "None"),
    }

    // Exclude regex
    match exclude_regex {
        Some(regex) => println!("{}: {}", "Exclude Regex".bold(), regex.to_string()),
        None => println!("{}: {}", "Exclude Regex".bold(), "None"),
    }

    println!("{}", "=".repeat(40).cyan());
    println!();
}

fn apply_highlighting<'a>(
    line: &'a str,
    highlight_rules: &[HighlightRule],
) -> Cow<'a, str> {
    let mut processed_line: Cow<'a, str> = Cow::Borrowed(line);

    for rule in highlight_rules {
        if rule.regex.is_match(&processed_line) {
            processed_line = Cow::Owned(
                rule.regex.replace_all(&processed_line, |caps: &regex::Captures| {
                    let matched = &caps[0];
                    apply_style(matched, &rule.color, &rule.style).to_string()
                })
                .into_owned(),
            );
        }
    }

    processed_line
}

fn apply_style(text: &str, text_color: &Color, text_style: &Style) -> ColoredString {
    let colored_text: ColoredString = match text_color {
        Color::Red => text.red(),
        Color::BrightRed => text.bright_red(),
        Color::Green => text.green(),
        Color::Yellow => text.yellow(),
        Color::Blue => text.blue(),
    };

    let styled_text: ColoredString = match text_style {
        Style::Bold => colored_text.bold(),
        Style::Italic => colored_text.italic(),
        Style::Underline => colored_text.underline(),
        Style::Normal => colored_text,
    };

    return styled_text;
}
