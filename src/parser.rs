use crate::models::{Match, Match::*, MatchType::*};
use crate::patterns::PHONETIC_PATTERNS;

fn is_vowel(c: char) -> bool {
    "aeiou".contains(c.to_ascii_lowercase())
}

fn is_consonant(c: char) -> bool {
    "bcdfghjklmnpqrstvwxyz".contains(c.to_ascii_lowercase())
}

fn is_punctuation(c: char) -> bool {
    !is_vowel(c) && !is_consonant(c)
}

fn conditional_lowercase(c: char) -> char {
    const CASE_SENSITIVE_CHARS: &'static str = "oiudgjnrstyz";
    let lowercase_c = c.to_ascii_lowercase();
    if CASE_SENSITIVE_CHARS.contains(lowercase_c) {
        c
    } else {
        lowercase_c
    }
}

impl Match {
    fn does_match(&self, prefix: char, suffix: char) -> bool {
        match *self {
            PrefixIs(Vowel) => is_vowel(prefix),
            PrefixIsNot(Vowel) => !is_vowel(prefix),
            PrefixIs(Consonant) => is_consonant(prefix),
            PrefixIsNot(Consonant) => !is_consonant(prefix),
            PrefixIs(Punctuation) => is_punctuation(prefix),
            PrefixIsNot(Punctuation) => !is_punctuation(prefix),
            PrefixIs(Char(c)) => (c == prefix),
            PrefixIsNot(Char(c)) => (c != prefix),
            SuffixIs(Vowel) => is_vowel(suffix),
            SuffixIsNot(Vowel) => !is_vowel(suffix),
            SuffixIs(Consonant) => is_consonant(suffix),
            SuffixIsNot(Consonant) => !is_consonant(suffix),
            SuffixIs(Punctuation) => is_punctuation(suffix),
            SuffixIsNot(Punctuation) => !is_punctuation(suffix),
            SuffixIs(Char(c)) => (c == suffix),
            SuffixIsNot(Char(c)) => (c != suffix),
        }
    }
}

pub fn parse(raw_input: &str) -> String {
    let input: String = raw_input.chars().map(conditional_lowercase).collect();

    let mut prefix = ' ';
    let mut current_input = &input[0..];
    let mut output = String::new();

    while !current_input.is_empty() {
        let match_result = PHONETIC_PATTERNS
            .iter()
            .filter(|p| current_input.starts_with(p.find))
            .max_by_key(|p| p.find.len());

        if let Some(pattern) = match_result {
            let suffix = current_input.chars().nth(pattern.find.len()).unwrap_or(' ');

            let matched_rule = pattern.rules.iter().find(|rule| {
                rule.when_matches
                    .iter()
                    .all(|m| m.does_match(prefix, suffix))
            });

            let replacement = match matched_rule {
                Some(rule) => rule.replace_with,
                None => pattern.default_replacement,
            };

            output.push_str(replacement);
            prefix = current_input.chars().nth(pattern.find.len() - 1).unwrap();
            current_input = &current_input[pattern.find.len()..];
        } else {
            prefix = current_input.chars().next().unwrap();
            output.push(prefix);
            current_input = &current_input[1..];
        }
    }

    output
}
