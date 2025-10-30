use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::IsNil};

impl<Lang> GrammarRuleDescribe for IsNil<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
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
