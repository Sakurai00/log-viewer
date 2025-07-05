use anyhow::Result;
use clap::Parser;
use colored::{self, ColoredString, Colorize};
use linemux::MuxedLines;
use regex::Regex;
use std::borrow::Cow;

mod debug;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, value_parser, num_args=1..)]
    log_files: Option<Vec<String>>,
    #[arg(short = 'd', long = "disable-preset-excludes")]
    disable_preset_excludes: bool,
    #[arg(short, long, value_parser, num_args=1..)]
    exclude_words: Option<Vec<String>>,
    #[arg(short, long, value_parser, num_args=1..)]
    include_words: Option<Vec<String>>,
    #[arg(long)]
    debug: bool,
}

const PRESET_EXCLUDE_WORDS: &[&str] = &["aaa", "bbb", "ccc"];
const DEFAULT_LOG_FILES: &[&str] = &["/var/log/messages"];

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
    Cyan,
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut log_reader = get_log_reader(&args.log_files).await?;

    let include_regex = build_include_regex(args.include_words)?;
    let exclude_regex = build_exclude_regex(args.exclude_words, args.disable_preset_excludes)?;
    let highlight_rules = get_highlight_rules()?;

    if args.debug {
        debug::print_debug_info(&args.log_files, &include_regex, &exclude_regex, DEFAULT_LOG_FILES);
    }

    while let Ok(Some(line)) = log_reader.next_line().await {
        let line = line.line();

        if should_display_line(line, &include_regex, &exclude_regex) {
            let highlighted_line = apply_highlighting(line, &highlight_rules);
            println!("{highlighted_line}");
        }
    }

    Ok(())
}

async fn get_log_reader(log_files: &Option<Vec<String>>) -> Result<MuxedLines> {
    let mut log_reader = MuxedLines::new()?;

    match log_files {
        Some(log_files) => {
            for file in log_files {
                log_reader.add_file(file).await?;
            }
        }
        None => {
            for file in DEFAULT_LOG_FILES {
                log_reader.add_file(file).await?;
            }
        }
    };

    Ok(log_reader)
}

fn build_include_regex(words: Option<Vec<String>>) -> Result<Option<Regex>> {
    let patterns: Vec<String> = words
        .into_iter()
        .flatten()
        .map(|word| regex::escape(&word))
        .collect();

    if patterns.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Regex::new(&patterns.join("|"))?))
    }
}

fn build_exclude_regex(words: Option<Vec<String>>, disable_preset_excludes: bool) -> Result<Option<Regex>> {
    let user_words = words.into_iter().flatten();

    let preset_words = (!disable_preset_excludes)
        .then_some(PRESET_EXCLUDE_WORDS.iter().map(|&s| s.to_string()))
        .into_iter()
        .flatten();

    let patterns: Vec<String> = user_words
        .chain(preset_words)
        .map(|word| regex::escape(&word))
        .collect();

    if patterns.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Regex::new(&patterns.join("|"))?))
    }
}

fn get_highlight_rules() -> Result<Vec<HighlightRule>> {
    let rules = vec![
        HighlightRule {
            regex: Regex::new(&CRITICAL_WORDS.join("|"))?,
            color: Color::BrightRed,
            style: Style::Bold,
        },
        HighlightRule {
            regex: Regex::new(&WARN_WORDS.join("|"))?,
            color: Color::Yellow,
            style: Style::Underline,
        },
        HighlightRule {
            regex: Regex::new(&INFO_WORDS.join("|"))?,
            color: Color::Cyan,
            style: Style::Normal,
        },
    ];
    Ok(rules)
}

fn should_display_line(line: &str, include_regex: &Option<Regex>, exclude_regex: &Option<Regex>) -> bool {
    // 除外フィルター: 除外ルールがないか、除外ルールに含まれない場合は表示。除外ルールに含まれる場合は表示させない。
    let passes_exclusion_filter = exclude_regex.as_ref().is_none_or(|re| !re.is_match(line));
    // 包含フィルター: 包含ルールがないか、包含ルールに含まれる場合は表示。包含ルールに含まれない場合は表示させない。
    let passes_inclusion_filter = include_regex.as_ref().is_none_or(|re| re.is_match(line));

    // 両方のフィルターを通過した場合のみ表示する
    passes_exclusion_filter && passes_inclusion_filter
}

fn apply_highlighting<'a>(line: &'a str, rules: &[HighlightRule]) -> Cow<'a, str> {
    let mut line: Cow<'a, str> = Cow::Borrowed(line);

    for rule in rules {
        line = Cow::Owned(
            rule.regex
                .replace_all(&line, |caps: &regex::Captures| {
                    let matched_word = &caps[0];
                    apply_style(matched_word, &rule.color, &rule.style).to_string()
                })
                .into_owned(),
        );
    }

    line
}

fn apply_style(text: &str, text_color: &Color, text_style: &Style) -> ColoredString {
    let colored_text: ColoredString = match text_color {
        Color::Red => text.red(),
        Color::BrightRed => text.bright_red(),
        Color::Green => text.green(),
        Color::Yellow => text.yellow(),
        Color::Blue => text.blue(),
        Color::Cyan => text.cyan(),
    };

    let styled_text: ColoredString = match text_style {
        Style::Bold => colored_text.bold(),
        Style::Italic => colored_text.italic(),
        Style::Underline => colored_text.underline(),
        Style::Normal => colored_text,
    };

    styled_text
}
