use anyhow::Result;
use colored::{self, ColoredString, Colorize};
use regex::Regex;
use std::borrow::Cow;

use crate::{CRITICAL_WORDS, INFO_WORDS, WARN_WORDS};

#[allow(dead_code)]
pub enum Color {
    Red,
    BrightRed,
    Green,
    Yellow,
    Blue,
    Cyan,
}

#[allow(dead_code)]
pub enum Style {
    Bold,
    Italic,
    Underline,
    Normal,
}

pub struct HighlightRule {
    pub regex: Regex,
    pub color: Color,
    pub style: Style,
}

pub fn get_highlight_rules() -> Result<Vec<HighlightRule>> {
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

pub fn apply_highlighting<'a>(line: &'a str, rules: &[HighlightRule]) -> Cow<'a, str> {
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
