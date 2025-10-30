use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, types::Sink};

impl<Lang> GrammarRuleDescribe for Sink<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Keyword::Sink.into(), Symbol::sqbrack(Symbol::Type)].into(),
            "Sink Type",
        )
    }
}
