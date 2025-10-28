use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Raise};

impl<Lang> GrammarRuleDescribe for Raise<Lang>
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
