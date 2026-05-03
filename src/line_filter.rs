use anyhow::Result;
use regex::Regex;

use crate::constants::PRESET_EXCLUDE_WORDS;
use crate::word_pattern::build_word_pattern;

pub struct LineFilter {
    include_regex: Option<Regex>,
    exclude_regex: Option<Regex>,
}

impl LineFilter {
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
            include_regex: build_word_pattern(&include_words.unwrap_or_default())?,
            exclude_regex: build_word_pattern(&exclude_word_list)?,
        })
    }

    pub fn allows(&self, line: &str) -> bool {
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
    use super::LineFilter;

    #[test]
    fn builds_include_regex_from_words() {
        let filter = LineFilter::new(
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
        let regex = filter.include_regex().as_ref().unwrap();
        assert!(regex.is_match("this is a critical error"));
        assert!(regex.is_match("a fatal exception occurred"));
        assert!(!regex.is_match("this is just info"));
    }

    #[test]
    fn builds_exclude_regex_with_user_and_preset_words() {
        let filter = LineFilter::new(
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
        let regex = filter.exclude_regex().as_ref().unwrap();
        assert!(regex.is_match("this is a debug message"));
        assert!(regex.is_match("filter out this spam"));
        assert!(regex.is_match("verbose logging enabled"));
        assert!(regex.is_match("this is an aaa message"));
        assert!(!regex.is_match("this is a regular message"));
    }

    #[test]
    fn should_display_line_with_various_scenarios() {
        let filter = LineFilter::new(
            Some(vec!["success".to_string(), "approved".to_string()]),
            Some(vec!["temp".to_string(), "interim".to_string()]),
            false,
        )
        .unwrap();

        assert!(filter.allows("operation was a success"));
        assert!(!filter.allows("operation failed"));
        assert!(!filter.allows("interim success report"));

        let include_only_filter = LineFilter::new(Some(vec!["approved".to_string()]), None, true).unwrap();
        assert!(include_only_filter.allows("request approved"));

        let exclude_only_filter = LineFilter::new(None, Some(vec!["temp".to_string()]), true).unwrap();
        assert!(!exclude_only_filter.allows("this is a temp file"));

        let no_rules_filter = LineFilter::new(None, None, true).unwrap();
        assert!(no_rules_filter.allows("any other message"));
        assert!(!no_rules_filter.allows(""));
    }
}
