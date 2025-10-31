use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, types::Sink};

impl<Lang> GrammarRuleDescribe for Sink<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Keyword::Sink.into(), Symbol::sqbrack(Symbol::Type)].into(),
            "Sink Type",
        )
    }
}
