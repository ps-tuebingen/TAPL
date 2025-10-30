use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Nothing};

impl<Lang> GrammarRuleDescribe for Nothing<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Nothing.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(vec![Symbol::Term]),
            ]
            .into(),
            "Nothing",
        )
    }
}
