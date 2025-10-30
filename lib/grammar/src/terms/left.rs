use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Left};

impl<Lang> GrammarRuleDescribe for Left<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Left.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(vec![Symbol::Term]),
            ]
            .into(),
            "Left Injection",
        )
    }
}
