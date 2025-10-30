use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Nil};

impl<Lang> GrammarRuleDescribe for Nil<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Nil.into(),
                Symbol::sqbrack(Symbol::Term),
                Symbol::paren(vec![Symbol::Term]),
            ]
            .into(),
            "Empty List",
        )
    }
}
