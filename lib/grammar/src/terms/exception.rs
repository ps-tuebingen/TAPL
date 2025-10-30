use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Exception};

impl<Lang> GrammarRuleDescribe for Exception<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Err.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(vec![Symbol::Term]),
            ]
            .into(),
            "Exception",
        )
    }
}
