use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Tail};

impl<Lang> GrammarRuleDescribe for Tail<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Tail, Some(Symbol::Type), vec![Symbol::Term]),
            "List Tail",
        )
    }
}
