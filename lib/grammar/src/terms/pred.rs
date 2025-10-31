use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Pred};

impl<Lang> GrammarRuleDescribe for Pred<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Keyword::Pred.into(), Symbol::paren(Symbol::Term)].into(),
            "Pred",
        )
    }
}
