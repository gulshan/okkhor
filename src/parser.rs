use crate::models::{Match, Match::*, MatchType::*, Pattern};
use crate::patterns::PHONETIC_PATTERNS;

const fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}

const fn is_consonant(c: char) -> bool {
    !is_vowel(c) && c.is_ascii_alphabetic()
}

const fn is_punctuation(c: char) -> bool {
    !c.is_ascii_alphabetic()
}

fn conditional_lowercase(c: char) -> char {
    const CASE_SENSITIVE_CHARS: &str = "oiudgjnrstyz";
    let lowercase_c = c.to_ascii_lowercase();
    if CASE_SENSITIVE_CHARS.contains(lowercase_c) {
        c
    } else {
        lowercase_c
    }
}

impl Match {
    const fn does_match(&self, prefix: char, suffix: char) -> bool {
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
    let mut output = String::with_capacity(input.len());

    while !current_input.is_empty() {
        let match_result = PHONETIC_PATTERNS
            .iter()
            .filter(|p| current_input.starts_with(p.find))
            .max_by_key(|p| p.find.len());

        match match_result {
            Some(Pattern {
                find,
                rules,
                default_replacement,
            }) => {
                if rules.is_empty() {
                    output.push_str(default_replacement);
                } else {
                    let suffix = current_input.chars().nth(find.len()).unwrap_or(' ');

                    let matched_rule = rules.into_iter().find(|rule| {
                        rule.when_matches
                            .iter()
                            .all(|m| m.does_match(prefix, suffix))
                    });

                    match matched_rule {
                        Some(rule) => output.push_str(rule.replace_with),
                        None => output.push_str(default_replacement),
                    };
                }
                prefix = find.chars().last().unwrap();
                current_input = &current_input[find.len()..];
            }
            None => {
                prefix = current_input.chars().next().unwrap();
                output.push(prefix);
                current_input = &current_input[1..];
            }
        }
    }

    output
}
