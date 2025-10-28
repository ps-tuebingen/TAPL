use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, types::Reference};

impl<Lang> GrammarRuleDescribe for Reference<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Reference, Some(Symbol::Type), vec![]),
            "Reference Type",
        )
    }
}
