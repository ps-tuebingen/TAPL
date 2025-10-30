use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Unfold};

impl<Lang> GrammarRuleDescribe for Unfold<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Unfold.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(vec![Symbol::Term]),
            ]
            .into(),
            "Unfold",
        )
    }
}
