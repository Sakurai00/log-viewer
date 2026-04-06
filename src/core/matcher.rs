use anyhow::Result;
use regex::Regex;

pub fn build_literal_matcher(words: &[String]) -> Result<Option<Regex>> {
    compile_literal_matcher(words.iter().map(String::as_str))
}

pub fn build_literal_matcher_from_strs(words: &[&str]) -> Result<Option<Regex>> {
    compile_literal_matcher(words.iter().copied())
}

fn compile_literal_matcher<'a>(words: impl Iterator<Item = &'a str>) -> Result<Option<Regex>> {
    let patterns: Vec<String> = words
        .filter(|word| !word.is_empty())
        .map(regex::escape)
        .collect();

    if patterns.is_empty() {
        return Ok(None);
    }

    Ok(Some(Regex::new(&patterns.join("|"))?))
}

#[cfg(test)]
mod tests {
    use super::{build_literal_matcher, build_literal_matcher_from_strs};

    #[test]
    fn returns_none_for_empty_lists() {
        assert!(build_literal_matcher(&[]).unwrap().is_none());
        assert!(build_literal_matcher_from_strs(&[]).unwrap().is_none());
    }

    #[test]
    fn escapes_words_before_building_regex() {
        let words = vec!["foo.bar".to_string(), "warn+".to_string()];
        let matcher = build_literal_matcher(&words).unwrap().unwrap();

        assert!(matcher.is_match("foo.bar"));
        assert!(matcher.is_match("warn+"));
        assert!(!matcher.is_match("fooXbar"));
    }
}
