use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Raise};

impl<Lang> GrammarRuleDescribe for Raise<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Raise.into(),
                Symbol::sqbrack(Symbol::Term),
                Symbol::paren(vec![Symbol::Term]),
            ]
            .into(),
            "Raise",
        )
    }
}
