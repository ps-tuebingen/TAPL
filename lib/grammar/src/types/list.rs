use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, types::List};

impl<Lang> GrammarRuleDescribe for List<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Keyword::List.into(), Symbol::sqbrack(Symbol::Type)].into(),
            "List Type",
        )
    }
}
