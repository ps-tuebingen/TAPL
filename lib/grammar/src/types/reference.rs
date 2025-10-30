use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, types::Reference};

impl<Lang> GrammarRuleDescribe for Reference<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Keyword::Reference.into(), Symbol::sqbrack(Symbol::Type)].into(),
            "Reference Type",
        )
    }
}
