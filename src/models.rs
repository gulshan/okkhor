pub(crate) enum MatchType {
    Vowel,
    Consonant,
    Punctuation,
    Char(char),
}

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

impl Pattern {
    pub const fn simple_replace(find: &'static str, replace: &'static str) -> Pattern {
        Pattern {
            find: find,
            rules: &[],
            default_replacement: replace,
        }
    }
}
