use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, types::Optional};

impl<Lang> GrammarRuleDescribe for Optional<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Optional, Some(Symbol::Type), vec![]),
            "Option Type",
        )
    }
}
