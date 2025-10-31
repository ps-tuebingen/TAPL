use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::Keyword};
use syntax::{language::Language, types::Optional};

impl<Lang> GrammarRuleDescribe for Optional<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Keyword::Optional.into(), Symbol::sqbrack(Symbol::Type)].into(),
            "Option Type",
        )
    }
}
