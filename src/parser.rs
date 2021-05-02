use crate::{
    models::{Match, Match::*, MatchType::*, Pattern},
    patterns::PHONETIC_PATTERNS,
};
use std::collections::BTreeMap;

fn conditional_lowercase(c: char) -> char {
    const CASE_SENSITIVE_CHARS: &str = "oiudgjnrstyz";
    let lowercase_c = c.to_ascii_lowercase();
    if CASE_SENSITIVE_CHARS.contains(lowercase_c) {
        c
    } else {
        lowercase_c
    }
}

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

const fn does_match(_match: &Match, prefix: char, suffix: char) -> bool {
    match *_match {
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

pub struct Parser {
    patterns: BTreeMap<&'static str, &'static Pattern>,
}

impl Parser {
    pub fn new_phonetic() -> Parser {
        Self::new(PHONETIC_PATTERNS)
    }

    pub(crate) fn new(patterns_input: &'static [Pattern]) -> Parser {
        let patterns = patterns_input
            .iter()
            .map(|p| (p.find, p))
            .collect::<BTreeMap<_, _>>();
        Parser { patterns }
    }

    pub fn convert(&self, raw_input: &str) -> String {
        let input: String = raw_input.chars().map(conditional_lowercase).collect();

        let mut prefix = ' ';
        let mut current_input = &input[0..];
        let mut output = String::with_capacity(input.len() * 3);

        while !current_input.is_empty() {
            let match_result = self
                .patterns
                .range(..=current_input)
                .rev()
                .find(|(k, _)| current_input.starts_with(*k))
                .map(|(_, p)| p);

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
                                .all(|m| does_match(m, prefix, suffix))
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
}
