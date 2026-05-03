use anyhow::Result;
use regex::Regex;
use std::borrow::Cow;

use crate::line_highlighter::LineHighlighter;
use crate::line_filter::LineFilter;

pub struct LinePipeline {
    filter: LineFilter,
    line_highlighter: LineHighlighter,
}

impl LinePipeline {
    pub fn new(
        include_words: Option<Vec<String>>,
        exclude_words: Option<Vec<String>>,
        disable_preset_excludes: bool,
    ) -> Result<Self> {
        Ok(Self {
            filter: LineFilter::new(include_words, exclude_words, disable_preset_excludes)?,
            line_highlighter: LineHighlighter::new()?,
        })
    }

    pub fn process<'a>(&self, line: &'a str) -> Option<Cow<'a, str>> {
        if self.filter.allows(line) {
            Some(self.line_highlighter.apply(line))
        } else {
            None
        }
    }

    pub fn include_regex(&self) -> &Option<Regex> {
        self.filter.include_regex()
    }

    pub fn exclude_regex(&self) -> &Option<Regex> {
        self.filter.exclude_regex()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::{control, Colorize};

    #[test]
    fn new_uses_default_exclude_regex() {
        let pipeline = LinePipeline::new(None, None, false).unwrap();
        assert!(pipeline.include_regex().is_none());
        assert!(pipeline.exclude_regex().is_some());
    }

    #[test]
    fn new_keeps_include_and_default_exclude_regex() {
        let pipeline = LinePipeline::new(Some(vec!["include".to_string()]), None, false).unwrap();
        assert!(pipeline.include_regex().is_some());
        assert!(pipeline.exclude_regex().is_some());
    }

    #[test]
    fn new_with_excludes_keeps_exclude_regex() {
        let pipeline = LinePipeline::new(None, Some(vec!["exclude".to_string()]), false).unwrap();
        assert!(pipeline.include_regex().is_none());
        assert!(pipeline.exclude_regex().is_some());
    }

    #[test]
    fn new_with_all_patterns_sets_both() {
        let pipeline = LinePipeline::new(
            Some(vec!["include".to_string()]),
            Some(vec!["exclude".to_string()]),
            true,
        )
        .unwrap();
        assert!(pipeline.include_regex().is_some());
        assert!(pipeline.exclude_regex().is_some());
    }

    #[test]
    fn process_returns_input_when_no_filters_match() {
        let pipeline = LinePipeline::new(None, None, true).unwrap();
        let line = "this is a test line";
        assert_eq!(pipeline.process(line), Some(Cow::from(line)));
    }

    #[test]
    fn process_returns_input_when_include_matches() {
        let pipeline = LinePipeline::new(Some(vec!["test".to_string()]), None, true).unwrap();
        let line = "this is a test line";
        assert_eq!(pipeline.process(line), Some(Cow::from(line)));
    }

    #[test]
    fn process_returns_none_when_include_does_not_match() {
        let pipeline = LinePipeline::new(Some(vec!["other".to_string()]), None, true).unwrap();
        let line = "this is a test line";
        assert!(pipeline.process(line).is_none());
    }

    #[test]
    fn process_returns_none_when_exclude_matches() {
        let pipeline = LinePipeline::new(None, Some(vec!["test".to_string()]), true).unwrap();
        let line = "this is a test line";
        assert!(pipeline.process(line).is_none());
    }

    #[test]
    fn process_returns_input_when_exclude_does_not_match() {
        let pipeline = LinePipeline::new(None, Some(vec!["other".to_string()]), true).unwrap();
        let line = "this is a test line";
        assert_eq!(pipeline.process(line), Some(Cow::from(line)));
    }

    #[test]
    fn process_returns_none_when_include_and_exclude_both_match() {
        let pipeline =
            LinePipeline::new(Some(vec!["test".to_string()]), Some(vec!["line".to_string()]), true).unwrap();
        let line = "this is a test line";
        assert!(pipeline.process(line).is_none());
    }

    #[test]
    fn process_applies_highlighting_to_allowed_lines() {
        control::set_override(true);
        let pipeline = LinePipeline::new(None, None, true).unwrap();
        let line = "this is a foo line";
        let expected = "this is a ".to_string() + &"foo".bright_red().bold().to_string() + " line";
        assert_eq!(pipeline.process(line), Some(Cow::from(expected)));
        control::unset_override();
    }

    #[test]
    fn process_returns_none_for_empty_string() {
        let pipeline = LinePipeline::new(None, None, false).unwrap();
        let line = "";
        assert!(pipeline.process(line).is_none());
    }
}
