use anyhow::Result;
use regex::Regex;

use crate::PRESET_EXCLUDE_WORDS;

pub fn build_include_regex(words: Option<Vec<String>>) -> Result<Option<Regex>> {
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

pub fn build_exclude_regex(words: Option<Vec<String>>, disable_preset_excludes: bool) -> Result<Option<Regex>> {
    let user_words = words.into_iter().flatten();

    let preset_words = (!disable_preset_excludes)
        .then_some(PRESET_EXCLUDE_WORDS.iter().map(|&s| s.to_string()))
        .into_iter()
        .flatten();

    let all_patterns: Vec<String> = user_words
        .chain(preset_words)
        .map(|word| regex::escape(&word))
        .collect();

    if all_patterns.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Regex::new(&all_patterns.join("|"))?))
    }
}

pub fn should_display_line(line: &str, include_regex: &Option<Regex>, exclude_regex: &Option<Regex>) -> bool {
    let passes_exclusion_filter = exclude_regex.as_ref().is_none_or(|re| !re.is_match(line));
    let passes_inclusion_filter = include_regex.as_ref().is_none_or(|re| re.is_match(line));

    passes_exclusion_filter && passes_inclusion_filter
}
