use crate::models::{Match, MatchType};
use crate::patterns::PATTERNS;

fn is_vowel(c: char) -> bool {
    "aeiou".contains(c.to_ascii_lowercase())
}

fn is_consonant(c: char) -> bool {
    "bcdfghjklmnpqrstvwxyz".contains(c.to_ascii_lowercase())
}

fn is_punctuation(c: char) -> bool {
    !is_vowel(c) & !is_consonant(c)
}

fn is_case_sensitive(c: char) -> bool {
    "oiudgjnrstyz".contains(c.to_ascii_lowercase())
}

impl Match {
    fn does_match(&self, prefix: char, suffix: char) -> bool {
        match *self {
            Match::Prefix(MatchType::Vowel) => is_vowel(prefix),
            Match::PrefixNot(MatchType::Vowel) => !is_vowel(prefix),
            Match::Prefix(MatchType::Consonant) => is_consonant(prefix),
            Match::PrefixNot(MatchType::Consonant) => !is_consonant(prefix),
            Match::Prefix(MatchType::Punctuation) => is_punctuation(prefix),
            Match::PrefixNot(MatchType::Punctuation) => !is_punctuation(prefix),
            Match::Prefix(MatchType::Exact(c)) => (c == prefix),
            Match::PrefixNot(MatchType::Exact(c)) => (c != prefix),
            Match::Suffix(MatchType::Vowel) => is_vowel(suffix),
            Match::SuffixNot(MatchType::Vowel) => !is_vowel(suffix),
            Match::Suffix(MatchType::Consonant) => is_consonant(suffix),
            Match::SuffixNot(MatchType::Consonant) => !is_consonant(suffix),
            Match::Suffix(MatchType::Punctuation) => is_punctuation(suffix),
            Match::SuffixNot(MatchType::Punctuation) => !is_punctuation(suffix),
            Match::Suffix(MatchType::Exact(c)) => (c == suffix),
            Match::SuffixNot(MatchType::Exact(c)) => (c != suffix),
        }
    }
}

pub fn parse(raw_input: &str) -> String {
    let input: String = raw_input
        .chars()
        .map(|c| {
            if is_case_sensitive(c) {
                c
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();

    let mut prefix = ' ';
    let mut current_input = &input[0..];
    let mut output = String::new();

    while !current_input.is_empty() {
        let matched_opt = PATTERNS
            .iter()
            .filter(|p| current_input.starts_with(p.find))
            .max_by_key(|p| p.find.len());

        if let Some(matched) = matched_opt {
            let suffix = if current_input.len() > matched.find.len() {
                current_input.chars().nth(matched.find.len()).unwrap()
            } else {
                ' '
            };

            let matched_rule = matched
                .rules
                .iter()
                .find(|rule| rule.matches.iter().all(|m| m.does_match(prefix, suffix)));
            let replace = match matched_rule {
                Some(rule) => rule.replace,
                None => matched.default_replace,
            };

            output.push_str(replace);
            prefix = current_input.chars().nth(matched.find.len() - 1).unwrap();
            current_input = &current_input[matched.find.len()..];
        } else {
            prefix = current_input.chars().next().unwrap();
            output.push(prefix);
            current_input = &current_input[1..];
        }
    }

    output
}
