use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Raise};

impl<Lang> RuleDescribe for Raise<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Raise, Some(Symbol::Type), vec![Symbol::Value]),
            "Raise",
        )
    }
}
