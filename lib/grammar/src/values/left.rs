use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, values::Left};

impl<Lang> GrammarRuleDescribe for Left<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Left, Some(Symbol::Type), vec![Symbol::Value]),
            "Left Injection",
        )
    }
}
