use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::IsNil};

impl<Lang> GrammarRuleDescribe for IsNil<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                Keyword::IsNil.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(Symbol::Term),
            ]
            .into(),
            "Is Nil",
        )
    }
}
