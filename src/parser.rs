use crate::{
    models::{
        Match::{self, *},
        MatchType::*,
        Pattern, Patterns,
    },
    patterns::PHONETIC_PATTERNS,
};

const fn conditional_lowercase(c: char) -> char {
    let lowercase_c = c.to_ascii_lowercase();
    if matches!(
        lowercase_c,
        'o' | 'i' | 'u' | 'd' | 'g' | 'j' | 'n' | 'r' | 's' | 't' | 'y' | 'z'
    ) {
        c
    } else {
        lowercase_c
    }
}

const fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

const fn is_consonant(c: char) -> bool {
    !is_vowel(c) && c.is_ascii_alphabetic()
}

const fn is_punctuation(c: char) -> bool {
    !c.is_ascii_alphabetic()
}

const fn does_match(_match: Match, prefix: char, suffix: char) -> bool {
    match _match {
        PrefixIs(Vowel) => is_vowel(prefix),
        PrefixIsNot(Vowel) => !is_vowel(prefix),
        PrefixIs(Consonant) => is_consonant(prefix),
        PrefixIsNot(Consonant) => !is_consonant(prefix),
        PrefixIs(Punctuation) => is_punctuation(prefix),
        PrefixIsNot(Punctuation) => !is_punctuation(prefix),
        PrefixIs(Number) => prefix.is_ascii_digit(),
        PrefixIsNot(Number) => !prefix.is_ascii_digit(),
        PrefixIs(Char(c)) => c == prefix,
        PrefixIsNot(Char(c)) => c != prefix,
        SuffixIs(Vowel) => is_vowel(suffix),
        SuffixIsNot(Vowel) => !is_vowel(suffix),
        SuffixIs(Consonant) => is_consonant(suffix),
        SuffixIsNot(Consonant) => !is_consonant(suffix),
        SuffixIs(Punctuation) => is_punctuation(suffix),
        SuffixIsNot(Punctuation) => !is_punctuation(suffix),
        SuffixIs(Number) => suffix.is_ascii_digit(),
        SuffixIsNot(Number) => !suffix.is_ascii_digit(),
        SuffixIs(Char(c)) => c == suffix,
        SuffixIsNot(Char(c)) => c != suffix,
    }
}

pub(crate) fn get_replacement(pattern: &Pattern, prefix: char, suffix: char) -> &'static str {
    pattern
        .rules
        .iter()
        .find(|rule| {
            rule.when_matches
                .iter()
                .all(|m| does_match(m.clone(), prefix, suffix))
        })
        .map_or(pattern.default_replacement, |rule| rule.replace_with)
}

pub struct Parser {
    pub(crate) patterns: Patterns,
}

impl Parser {
    pub fn new_phonetic() -> Self {
        Self {
            patterns: Patterns::new(PHONETIC_PATTERNS),
        }
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
            match self.patterns.find_pattern(input) {
                Some(pattern) => {
                    let suffix = input.chars().nth(pattern.find.len()).unwrap_or(' ');
                    output.push_str(get_replacement(pattern, prefix, suffix));
                    pattern.find.chars().last().map(|c| prefix = c);
                    input = &input[pattern.find.len()..];
                }
                None => {
                    input.chars().next().map(|c| prefix = c);
                    output.push(prefix);
                    input = &input[1..];
                }
            }
        }
    }
}
