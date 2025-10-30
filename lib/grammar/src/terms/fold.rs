use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Fold};

impl<Lang> GrammarRuleDescribe for Fold<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Fold.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(Symbol::Term),
            ]
            .into(),
            "Fold",
        )
    }
}
