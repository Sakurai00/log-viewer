use anyhow::Result;
use regex::Regex;
use std::borrow::Cow;

use super::filtering::FilterEngine;
use super::highlighting::Highlighter;

pub struct LineFormatter {
    filter_engine: FilterEngine,
    highlighter: Highlighter,
}

impl LineFormatter {
    pub fn new(
        include_words: Option<Vec<String>>,
        exclude_words: Option<Vec<String>>,
        disable_preset_excludes: bool,
    ) -> Result<Self> {
        Ok(Self {
            filter_engine: FilterEngine::new(include_words, exclude_words, disable_preset_excludes)?,
            highlighter: Highlighter::new()?,
        })
    }

    pub fn process_line<'a>(&self, line: &'a str) -> Option<Cow<'a, str>> {
        if self.filter_engine.should_display(line) {
            Some(self.highlighter.highlight(line))
        } else {
            None
        }
    }

    pub fn get_include_regex(&self) -> &Option<Regex> {
        self.filter_engine.include_regex()
    }

    pub fn get_exclude_regex(&self) -> &Option<Regex> {
        self.filter_engine.exclude_regex()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::{control, Colorize};

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
        control::set_override(true);
        let formatter = LineFormatter::new(None, None, true).unwrap();
        let line = "this is a foo line";
        let expected = "this is a ".to_string() + &"foo".bright_red().bold().to_string() + " line";
        assert_eq!(formatter.process_line(line), Some(Cow::from(expected)));
        control::unset_override();
    }

    #[test]
    fn test_process_line_empty_string() {
        let formatter = LineFormatter::new(None, None, false).unwrap();
        let line = "";
        assert!(formatter.process_line(line).is_none());
    }
}
