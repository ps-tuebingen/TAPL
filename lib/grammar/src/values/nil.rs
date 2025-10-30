use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Nil};

impl<Lang> GrammarRuleDescribe for Nil<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Nil.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(Symbol::Value),
            ]
            .into(),
            "Empty List",
        )
    }
}
