use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Succ};

impl<Lang> GrammarRuleDescribe for Succ<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Keyword::Succ.into(), Symbol::paren(Symbol::Term)].into(),
            "Succ",
        )
    }
}
