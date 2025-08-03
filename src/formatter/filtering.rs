use anyhow::Result;
use regex::Regex;

use crate::constants::PRESET_EXCLUDE_WORDS;

pub fn build_include_regex(words: Option<Vec<String>>) -> Result<Option<Regex>> {
    let word_list: Vec<String> = words.into_iter().flatten().collect();
    compile_words_to_regex(&word_list)
}

pub fn build_exclude_regex(words: Option<Vec<String>>, disable_preset_excludes: bool) -> Result<Option<Regex>> {
    let mut word_list = words.unwrap_or_default();

    if !disable_preset_excludes {
        word_list.extend(PRESET_EXCLUDE_WORDS.iter().map(|&s| s.to_string()));
    }

    compile_words_to_regex(&word_list)
}

fn compile_words_to_regex(words: &[String]) -> Result<Option<Regex>> {
    if words.is_empty() {
        return Ok(None);
    }
    let patterns: Vec<String> = words.iter().map(|word| regex::escape(word)).collect();
    Ok(Some(Regex::new(&patterns.join("|"))?))
}

pub fn should_display_line(line: &str, include_regex: &Option<Regex>, exclude_regex: &Option<Regex>) -> bool {
    let passes_exclusion_filter = exclude_regex.as_ref().is_none_or(|re| !re.is_match(line));
    let passes_inclusion_filter = include_regex.as_ref().is_none_or(|re| re.is_match(line));

    passes_exclusion_filter && passes_inclusion_filter
}
