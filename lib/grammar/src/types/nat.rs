use crate::{GrammarRule, GrammarRuleDescribe, symbols::Keyword};
use syntax::{language::Language, types::Nat};

impl<Lang> GrammarRuleDescribe for Nat<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Keyword::Nat.into(), "Natural Numbers")
    }
}
