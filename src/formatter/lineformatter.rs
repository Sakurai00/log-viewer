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
