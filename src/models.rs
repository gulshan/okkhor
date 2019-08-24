pub enum MatchType {
    Vowel,
    Consonant,
    Punctuation,
    Char(char),
}

pub enum Match {
    PrefixIs(MatchType),
    SuffixIs(MatchType),
    PrefixIsNot(MatchType),
    SuffixIsNot(MatchType),
}

pub struct Rule {
    pub when_matches: &'static [Match],
    pub replace_with: &'static str,
}

pub struct Pattern {
    pub find: &'static str,
    pub rules: &'static [Rule],
    pub default_replacement: &'static str,
}

const NO_RULES: &[Rule] = &[];

impl Pattern {
    pub const fn simple_replace(find: &'static str, replace: &'static str) -> Pattern {
        Pattern {
            find: find,
            rules: NO_RULES,
            default_replacement: replace,
        }
    }
}
