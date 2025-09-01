use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::{language::Language, types::Nat};

impl<Lang> RuleDescribe for Nat<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::Nat.into(), "Natural Numbers")
    }
}
