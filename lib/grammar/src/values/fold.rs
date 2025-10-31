use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Fold};

impl<Lang> GrammarRuleDescribe for Fold<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                Keyword::Fold.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(Symbol::Value),
            ]
            .into(),
            "Fold",
        )
    }
}
