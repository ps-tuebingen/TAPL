use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Nil};

impl<Lang> GrammarRuleDescribe for Nil<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Nil, Some(Symbol::Type), vec![Symbol::Value]),
            "Empty List",
        )
    }
}
