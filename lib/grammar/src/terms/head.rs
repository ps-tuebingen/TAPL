use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Head};

impl<Lang> GrammarRuleDescribe for Head<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                Keyword::Head.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(Symbol::Term),
            ]
            .into(),
            "List Head",
        )
    }
}
