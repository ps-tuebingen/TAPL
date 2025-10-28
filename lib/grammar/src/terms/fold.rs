use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Fold};

impl<Lang> GrammarRuleDescribe for Fold<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Fold, Some(Symbol::Type), vec![Symbol::Term]),
            "Fold",
        )
    }
}
