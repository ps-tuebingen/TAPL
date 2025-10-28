use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::IsNil};

impl<Lang> GrammarRuleDescribe for IsNil<Lang>
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
