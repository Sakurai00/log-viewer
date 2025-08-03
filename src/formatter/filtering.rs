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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_include_regex_with_larger_list() {
        let words = Some(vec![
            "error".to_string(),
            "warning".to_string(),
            "critical".to_string(),
            "fatal".to_string(),
            "exception".to_string(),
        ]);
        let regex = build_include_regex(words).unwrap().unwrap();
        assert_eq!(regex.is_match("this is a critical error"), true);
        assert_eq!(regex.is_match("a fatal exception occurred"), true);
        assert_eq!(regex.is_match("this is just info"), false);
    }

    #[test]
    fn test_build_exclude_regex_with_user_and_preset_words() {
        let words = Some(vec![
            "debug".to_string(),
            "spam".to_string(),
            "noise".to_string(),
            "verbose".to_string(),
            "temp".to_string(),
        ]);
        let regex = build_exclude_regex(words, false).unwrap().unwrap();
        assert_eq!(regex.is_match("this is a debug message"), true);
        assert_eq!(regex.is_match("filter out this spam"), true);
        assert_eq!(regex.is_match("verbose logging enabled"), true);
        // Assuming "aaa" is a preset exclude word
        assert_eq!(regex.is_match("this is an aaa message"), true);
        assert_eq!(regex.is_match("this is a regular message"), false);
    }

    #[test]
    fn test_should_display_line_with_various_scenarios() {
        let include_regex = build_include_regex(Some(vec!["success".to_string(), "approved".to_string()])).unwrap();
        let exclude_regex = build_exclude_regex(Some(vec!["temp".to_string(), "interim".to_string()]), false).unwrap();

        // Matches include, not exclude -> should display
        assert_eq!(should_display_line(
            "operation was a success",
            &include_regex,
            &exclude_regex
        ), true);
        // Doesn't match include -> should not display
        assert_eq!(should_display_line("operation failed", &include_regex, &exclude_regex), false);
        // Matches both include and exclude -> should not display
        assert_eq!(should_display_line(
            "interim success report",
            &include_regex,
            &exclude_regex
        ), false);
        // No include regex, but matches exclude -> should not display
        assert_eq!(should_display_line("this is a temp file", &None, &exclude_regex), false);
        // No exclude regex, but matches include -> should display
        assert_eq!(should_display_line("request approved", &include_regex, &None), true);
        // No rules -> should display
        assert_eq!(should_display_line("any other message", &None, &None), true);
    }
}
