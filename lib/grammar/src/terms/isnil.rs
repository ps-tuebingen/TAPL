use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::IsNil};

impl<Lang> RuleDescribe for IsNil<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::IsNil, Some(Symbol::Type), vec![Symbol::Term]),
            "Is Nil",
        )
    }
}
