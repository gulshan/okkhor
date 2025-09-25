use std::collections::BTreeMap;

#[derive(Clone)]
pub(crate) enum MatchType {
    Vowel,
    Consonant,
    Punctuation,
    Number,
    Char(char),
}

#[derive(Clone)]
pub(crate) enum Match {
    PrefixIs(MatchType),
    SuffixIs(MatchType),
    PrefixIsNot(MatchType),
    SuffixIsNot(MatchType),
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

const NO_RULES: &[Rule] = &[];

impl Pattern {
    pub const fn simple_replace(find: &'static str, replace: &'static str) -> Pattern {
        Pattern {
            find,
            rules: NO_RULES,
            default_replacement: replace,
        }
    }
}

pub(crate) struct Patterns(BTreeMap<&'static str, &'static Pattern>);

impl Patterns {
    pub(crate) fn new(patterns_input: &'static [Pattern]) -> Self {
        let patterns = patterns_input
            .iter()
            .map(|p| (p.find, p))
            .collect();
        Self(patterns)
    }

    pub(crate) fn find_pattern(&self, input: &str) -> Option<&Pattern> {
        self.0
            .range(..=input)
            .rfind(|(&k, _)| input.starts_with(k))
            .map(|(_, &p)| p)
    }
}
