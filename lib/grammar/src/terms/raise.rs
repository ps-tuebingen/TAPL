use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Raise};

impl<Lang> RuleDescribe for Raise<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Raise, Some(Symbol::Term), vec![Symbol::Term]),
            "Raise",
        )
    }
}
