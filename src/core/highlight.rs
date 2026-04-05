use anyhow::Result;
use colored::{self, ColoredString, Colorize};
use regex::Regex;
use std::borrow::Cow;

use crate::constants::{CRITICAL_WORDS, INFO_WORDS, WARN_WORDS};

use super::matcher::build_literal_matcher_from_strs;

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

pub struct Highlighter {
    rules: Vec<HighlightRule>,
}

impl Highlighter {
    pub fn new() -> Result<Self> {
        Ok(Self {
            rules: vec![
                HighlightRule {
                    regex: build_literal_matcher_from_strs(CRITICAL_WORDS)?.unwrap(),
                    color: Color::BrightRed,
                    style: Style::Bold,
                },
                HighlightRule {
                    regex: build_literal_matcher_from_strs(WARN_WORDS)?.unwrap(),
                    color: Color::Yellow,
                    style: Style::Underline,
                },
                HighlightRule {
                    regex: build_literal_matcher_from_strs(INFO_WORDS)?.unwrap(),
                    color: Color::Cyan,
                    style: Style::Normal,
                },
            ],
        })
    }

    pub fn apply<'a>(&self, line: &'a str) -> Cow<'a, str> {
        let mut line: Cow<'a, str> = Cow::Borrowed(line);

        for rule in &self.rules {
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

    #[cfg(test)]
    pub fn rules(&self) -> &[HighlightRule] {
        &self.rules
    }
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

#[cfg(test)]
mod tests {
    use super::{apply_style, Color, Highlighter, Style};
    use colored::Colorize;

    #[test]
    fn creates_default_highlight_rules() {
        let highlighter = Highlighter::new().unwrap();
        assert_eq!(highlighter.rules().len(), 3);
    }

    #[test]
    fn apply_style_returns_colored_text() {
        let text = "test";
        let colored_text = apply_style(text, &Color::Red, &Style::Bold);
        assert_eq!(colored_text, "test".red().bold());
    }

    #[test]
    fn highlight_returns_input_when_no_match_exists() {
        let highlighter = Highlighter::new().unwrap();
        let line = "this is a normal line";
        let highlighted_line = highlighter.apply(line);
        assert_eq!(highlighted_line, line);
    }

    #[test]
    fn highlight_applies_configured_critical_rule() {
        let highlighter = Highlighter::new().unwrap();
        let line = "this is a foo line";
        let highlighted_line = highlighter.apply(line);
        assert_eq!(
            highlighted_line,
            "this is a ".to_string() + &apply_style("foo", &Color::BrightRed, &Style::Bold).to_string() + " line"
        );
    }

    #[test]
    fn highlight_applies_multiple_rules() {
        let highlighter = Highlighter::new().unwrap();
        let line = "foo warning success";
        let highlighted_line = highlighter.apply(line);
        assert_eq!(
            highlighted_line,
            apply_style("foo", &Color::BrightRed, &Style::Bold).to_string()
                + " "
                + &apply_style("warning", &Color::Yellow, &Style::Underline).to_string()
                + " "
                + &apply_style("success", &Color::Cyan, &Style::Normal).to_string()
        );
    }

    #[test]
    fn highlight_handles_empty_string() {
        let highlighter = Highlighter::new().unwrap();
        let line = "";
        let highlighted_line = highlighter.apply(line);
        assert_eq!(highlighted_line, "");
    }
}
