use anyhow::Result;
use regex::Regex;
use std::borrow::Cow;

use super::filtering::{build_exclude_regex, build_include_regex, should_display_line};
use super::highlighting::{apply_highlighting, get_highlight_rules, HighlightRule};

pub struct LineFormatter {
    include_regex: Option<Regex>,
    exclude_regex: Option<Regex>,
    highlight_rules: Vec<HighlightRule>,
}

impl LineFormatter {
    pub fn new(
        include_words: Option<Vec<String>>,
        exclude_words: Option<Vec<String>>,
        disable_preset_excludes: bool,
    ) -> Result<Self> {
        Ok(Self {
            include_regex: build_include_regex(include_words)?,
            exclude_regex: build_exclude_regex(exclude_words, disable_preset_excludes)?,
            highlight_rules: get_highlight_rules()?,
        })
    }

    pub fn process_line<'a>(&self, line: &'a str) -> Option<Cow<'a, str>> {
        if should_display_line(line, &self.include_regex, &self.exclude_regex) {
            Some(apply_highlighting(line, &self.highlight_rules))
        } else {
            None
        }
    }

    pub fn get_include_regex(&self) -> &Option<Regex> {
        &self.include_regex
    }

    pub fn get_exclude_regex(&self) -> &Option<Regex> {
        &self.exclude_regex
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;

    #[test]
    fn test_line_formatter_new_default() {
        let formatter = LineFormatter::new(None, None, false).unwrap();
        assert!(formatter.get_include_regex().is_none());
        assert!(formatter.get_exclude_regex().is_some());
    }

    #[test]
    fn test_line_formatter_new_with_includes() {
        let formatter = LineFormatter::new(Some(vec!["include".to_string()]), None, false).unwrap();
        assert!(formatter.get_include_regex().is_some());
        assert!(formatter.get_exclude_regex().is_some());
    }

    #[test]
    fn test_line_formatter_new_with_excludes() {
        let formatter = LineFormatter::new(None, Some(vec!["exclude".to_string()]), false).unwrap();
        assert!(formatter.get_include_regex().is_none());
        assert!(formatter.get_exclude_regex().is_some());
    }

    #[test]
    fn test_line_formatter_new_with_all() {
        let formatter = LineFormatter::new(
            Some(vec!["include".to_string()]),
            Some(vec!["exclude".to_string()]),
            true,
        )
        .unwrap();
        assert!(formatter.get_include_regex().is_some());
        assert!(formatter.get_exclude_regex().is_some());
    }

    #[test]
    fn test_process_line_no_filters() {
        let formatter = LineFormatter::new(None, None, true).unwrap();
        let line = "this is a test line";
        assert_eq!(formatter.process_line(line), Some(Cow::from(line)));
    }

    #[test]
    fn test_process_line_include_match() {
        let formatter = LineFormatter::new(Some(vec!["test".to_string()]), None, true).unwrap();
        let line = "this is a test line";
        assert_eq!(formatter.process_line(line), Some(Cow::from(line)));
    }

    #[test]
    fn test_process_line_include_no_match() {
        let formatter = LineFormatter::new(Some(vec!["other".to_string()]), None, true).unwrap();
        let line = "this is a test line";
        assert!(formatter.process_line(line).is_none());
    }

    #[test]
    fn test_process_line_exclude_match() {
        let formatter = LineFormatter::new(None, Some(vec!["test".to_string()]), true).unwrap();
        let line = "this is a test line";
        assert!(formatter.process_line(line).is_none());
    }

    #[test]
    fn test_process_line_exclude_no_match() {
        let formatter = LineFormatter::new(None, Some(vec!["other".to_string()]), true).unwrap();
        let line = "this is a test line";
        assert_eq!(formatter.process_line(line), Some(Cow::from(line)));
    }

    #[test]
    fn test_process_line_include_and_exclude_match() {
        let formatter =
            LineFormatter::new(Some(vec!["test".to_string()]), Some(vec!["line".to_string()]), true).unwrap();
        let line = "this is a test line";
        assert!(formatter.process_line(line).is_none());
    }

    #[test]
    fn test_process_line_highlighting() {
        let formatter = LineFormatter::new(None, None, true).unwrap();
        let line = "this is a CRITICAL line";
        let expected = "this is a ".to_string() + &"CRITICAL".bright_red().bold().to_string() + " line";
        assert_eq!(formatter.process_line(line), Some(Cow::from(expected)));
    }

    #[test]
    fn test_process_line_empty_string() {
        let formatter = LineFormatter::new(None, None, false).unwrap();
        let line = "";
        assert!(formatter.process_line(line).is_none());
    }
}
