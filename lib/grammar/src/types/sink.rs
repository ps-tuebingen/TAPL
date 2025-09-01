use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, types::Sink};

impl<Lang> RuleDescribe for Sink<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Sink, Some(Symbol::Type), vec![]),
            "Sink Type",
        )
    }
}
