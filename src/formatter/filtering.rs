use anyhow::Result;
use regex::Regex;

use crate::constants::PRESET_EXCLUDE_WORDS;

use super::matcher::build_literal_matcher;

pub struct FilterEngine {
    include_regex: Option<Regex>,
    exclude_regex: Option<Regex>,
}

impl FilterEngine {
    pub fn new(
        include_words: Option<Vec<String>>,
        exclude_words: Option<Vec<String>>,
        disable_preset_excludes: bool,
    ) -> Result<Self> {
        let mut exclude_word_list = exclude_words.unwrap_or_default();
        if !disable_preset_excludes {
            exclude_word_list.extend(PRESET_EXCLUDE_WORDS.iter().map(|&s| s.to_string()));
        }

        Ok(Self {
            include_regex: build_literal_matcher(&include_words.unwrap_or_default())?,
            exclude_regex: build_literal_matcher(&exclude_word_list)?,
        })
    }

    pub fn should_display(&self, line: &str) -> bool {
        if line.is_empty() {
            return false;
        }

        let passes_exclusion_filter = self.exclude_regex.as_ref().is_none_or(|re| !re.is_match(line));
        let passes_inclusion_filter = self.include_regex.as_ref().is_none_or(|re| re.is_match(line));

        passes_exclusion_filter && passes_inclusion_filter
    }

    pub fn include_regex(&self) -> &Option<Regex> {
        &self.include_regex
    }

    pub fn exclude_regex(&self) -> &Option<Regex> {
        &self.exclude_regex
    }
}

#[cfg(test)]
mod tests {
    use super::FilterEngine;

    #[test]
    fn builds_include_matcher_from_words() {
        let engine = FilterEngine::new(
            Some(vec![
            "error".to_string(),
            "warning".to_string(),
            "critical".to_string(),
            "fatal".to_string(),
            "exception".to_string(),
            ]),
            None,
            true,
        )
        .unwrap();
        let regex = engine.include_regex().as_ref().unwrap();
        assert!(regex.is_match("this is a critical error"));
        assert!(regex.is_match("a fatal exception occurred"));
        assert!(!regex.is_match("this is just info"));
    }

    #[test]
    fn builds_exclude_matcher_with_user_and_preset_words() {
        let engine = FilterEngine::new(
            None,
            Some(vec![
            "debug".to_string(),
            "spam".to_string(),
            "noise".to_string(),
            "verbose".to_string(),
            "temp".to_string(),
            ]),
            false,
        )
        .unwrap();
        let regex = engine.exclude_regex().as_ref().unwrap();
        assert!(regex.is_match("this is a debug message"));
        assert!(regex.is_match("filter out this spam"));
        assert!(regex.is_match("verbose logging enabled"));
        assert!(regex.is_match("this is an aaa message"));
        assert!(!regex.is_match("this is a regular message"));
    }

    #[test]
    fn should_display_line_with_various_scenarios() {
        let engine = FilterEngine::new(
            Some(vec!["success".to_string(), "approved".to_string()]),
            Some(vec!["temp".to_string(), "interim".to_string()]),
            false,
        )
        .unwrap();

        assert!(engine.should_display("operation was a success"));
        assert!(!engine.should_display("operation failed"));
        assert!(!engine.should_display("interim success report"));

        let include_only_engine =
            FilterEngine::new(Some(vec!["approved".to_string()]), None, true).unwrap();
        assert!(include_only_engine.should_display("request approved"));

        let exclude_only_engine = FilterEngine::new(None, Some(vec!["temp".to_string()]), true).unwrap();
        assert!(!exclude_only_engine.should_display("this is a temp file"));

        let no_rules_engine = FilterEngine::new(None, None, true).unwrap();
        assert!(no_rules_engine.should_display("any other message"));
        assert!(!no_rules_engine.should_display(""));
    }
}
