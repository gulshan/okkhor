use std::collections::BTreeMap;

#[derive(Clone)]
pub(crate) enum CharType {
    Vowel,
    Consonant,
    Punctuation,
    Number,
    Char(char),
}

pub(crate) enum Match {
    PrefixIs(CharType),
    SuffixIs(CharType),
    PrefixIsNot(CharType),
    SuffixIsNot(CharType),
}

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

impl Match {
    const fn satisfies(&self, prefix: char, suffix: char) -> bool {
        use CharType::*;
        use Match::*;
        match self {
            PrefixIs(Vowel) => is_vowel(prefix),
            PrefixIsNot(Vowel) => !is_vowel(prefix),
            PrefixIs(Consonant) => is_consonant(prefix),
            PrefixIsNot(Consonant) => !is_consonant(prefix),
            PrefixIs(Punctuation) => is_punctuation(prefix),
            PrefixIsNot(Punctuation) => !is_punctuation(prefix),
            PrefixIs(Number) => prefix.is_ascii_digit(),
            PrefixIsNot(Number) => !prefix.is_ascii_digit(),
            PrefixIs(Char(c)) => *c == prefix,
            PrefixIsNot(Char(c)) => *c != prefix,
            SuffixIs(Vowel) => is_vowel(suffix),
            SuffixIsNot(Vowel) => !is_vowel(suffix),
            SuffixIs(Consonant) => is_consonant(suffix),
            SuffixIsNot(Consonant) => !is_consonant(suffix),
            SuffixIs(Punctuation) => is_punctuation(suffix),
            SuffixIsNot(Punctuation) => !is_punctuation(suffix),
            SuffixIs(Number) => suffix.is_ascii_digit(),
            SuffixIsNot(Number) => !suffix.is_ascii_digit(),
            SuffixIs(Char(c)) => *c == suffix,
            SuffixIsNot(Char(c)) => *c != suffix,
        }
    }
}

pub(crate) struct Rule {
    pub when_matches: &'static [Match],
    pub replace_with: &'static str,
}

pub(crate) struct Pattern {
    pub find: &'static str,
    pub rules: &'static [Rule],
    pub default_replacement: &'static str,
}

impl Pattern {
    pub const fn simple_replace(find: &'static str, replace: &'static str) -> Pattern {
        Pattern {
            find,
            rules: &[],
            default_replacement: replace,
        }
    }

    pub(crate) fn get_replacement(&self, prefix: char, suffix: char) -> &'static str {
        self.rules
            .iter()
            .find(|rule| {
                rule.when_matches
                    .iter()
                    .all(|m| m.satisfies(prefix, suffix))
            })
            .map_or(self.default_replacement, |rule| rule.replace_with)
    }
}

pub(crate) struct Patterns(BTreeMap<&'static str, &'static Pattern>);

impl Patterns {
    pub(crate) fn new(patterns_input: &'static [Pattern]) -> Self {
        let patterns = patterns_input.iter().map(|p| (p.find, p)).collect();
        Self(patterns)
    }

    pub(crate) fn find_pattern(&self, input: &str) -> Option<&Pattern> {
        self.0
            .range(..=input)
            .rfind(|(&k, _)| input.starts_with(k))
            .map(|(_, &p)| p)
    }
}

pub struct Parser {
    pub(crate) patterns: Patterns,
}

impl Parser {
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
                    output.push_str(pattern.get_replacement(prefix, suffix));
                    if let Some(c) = pattern.find.chars().last() {
                        prefix = c;
                    }
                    input = &input[pattern.find.len()..];
                }
                None => {
                    if let Some(c) = input.chars().next() {
                        prefix = c;
                        output.push(c);
                    }
                    input = &input[1..];
                }
            }
        }
    }
}
