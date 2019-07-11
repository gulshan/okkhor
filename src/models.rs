pub enum MatchType {
    Vowel,
    Consonant,
    Punctuation,
    Exact(char),
}

pub enum Match {
    Prefix(MatchType),
    Suffix(MatchType),
    PrefixNot(MatchType),
    SuffixNot(MatchType),
}

pub struct Rule {
    pub matches: &'static [Match],
    pub replace: &'static str,
}

pub struct Pattern {
    pub find: &'static str,
    pub rules: &'static [Rule],
    pub default_replace: &'static str,
}

const NO_RULES: &'static [Rule] = &[];

impl Pattern {
    pub const fn simple_replace(find: &'static str, replace: &'static str) -> Pattern {
        Pattern {
            find: find,
            rules: NO_RULES,
            default_replace: replace,
        }
    }
}
