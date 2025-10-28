use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, types::List};

impl<Lang> GrammarRuleDescribe for List<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::List, Some(Symbol::Type), vec![]),
            "List Type",
        )
    }
}
