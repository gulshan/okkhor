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

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}

fn is_consonant(c: char) -> bool {
    !is_vowel(c) && c.is_ascii_alphabetic()
}

const fn is_punctuation(c: char) -> bool {
    !c.is_ascii_alphabetic()
}

fn does_match(_match: &Match, prefix: char, suffix: char) -> bool {
    match _match {
        PrefixIs(Vowel) => is_vowel(prefix),
        PrefixIsNot(Vowel) => !is_vowel(prefix),
        PrefixIs(Consonant) => is_consonant(prefix),
        PrefixIsNot(Consonant) => !is_consonant(prefix),
        PrefixIs(Punctuation) => is_punctuation(prefix),
        PrefixIsNot(Punctuation) => !is_punctuation(prefix),
        PrefixIs(Char(c)) => (*c == prefix),
        PrefixIsNot(Char(c)) => (*c != prefix),
        SuffixIs(Vowel) => is_vowel(suffix),
        SuffixIsNot(Vowel) => !is_vowel(suffix),
        SuffixIs(Consonant) => is_consonant(suffix),
        SuffixIsNot(Consonant) => !is_consonant(suffix),
        SuffixIs(Punctuation) => is_punctuation(suffix),
        SuffixIsNot(Punctuation) => !is_punctuation(suffix),
        SuffixIs(Char(c)) => (*c == suffix),
        SuffixIsNot(Char(c)) => (*c != suffix),
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
        let mut output = String::with_capacity(64);
        self.convert_into(raw_input, &mut output);
        output
    }

    pub fn convert_into(&self, raw_input: &str, output: &mut String) {
        let input: String = raw_input.chars().map(conditional_lowercase).collect();

        let mut prefix = ' ';
        let mut input = &input[0..];

        output.clear();
        while !input.is_empty() {
            match self.find_pattern(input) {
                Some(pattern) => {
                    output.push_str(pattern.get_replacement(input, prefix));
                    prefix = pattern.find.chars().last().unwrap();
                    input = &input[pattern.find.len()..];
                }
                None => {
                    prefix = input.chars().next().unwrap();
                    output.push(prefix);
                    input = &input[1..];
                }
            }
        }
    }

    pub(crate) fn find_pattern(&self, input: &str) -> Option<&Pattern> {
        self.patterns
            .range(..=input)
            .rfind(|(&k, _)| input.starts_with(k))
            .map(|(_, &p)| p)
    }
}

impl Pattern {
    pub(crate) fn get_replacement(&self, input: &str, prefix: char) -> &str {
        if self.rules.is_empty() {
            self.default_replacement
        } else {
            let suffix = input.chars().nth(self.find.len()).unwrap_or(' ');

            let matched_rule = self.rules.iter().find(|rule| {
                rule.when_matches
                    .iter()
                    .all(|m| does_match(m, prefix, suffix))
            });

            match matched_rule {
                Some(rule) => rule.replace_with,
                None => self.default_replacement,
            }
        }
    }
}
