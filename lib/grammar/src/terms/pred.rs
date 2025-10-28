use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Pred};

impl<Lang> GrammarRuleDescribe for Pred<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Pred, None, vec![Symbol::Term]),
            "Pred",
        )
    }
}
