use crate::{GrammarRuleDescribe, Rule, symbols::Keyword};
use syntax::{language::Language, types::Nat};

impl<Lang> GrammarRuleDescribe for Nat<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::Nat.into(), "Natural Numbers")
    }
}
