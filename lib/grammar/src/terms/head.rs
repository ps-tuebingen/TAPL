use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Head};

impl<Lang> GrammarRuleDescribe for Head<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Head, Some(Symbol::Type), vec![Symbol::Term]),
            "List Head",
        )
    }
}
