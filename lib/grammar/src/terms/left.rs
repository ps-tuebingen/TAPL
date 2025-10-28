use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Left};

impl<Lang> GrammarRuleDescribe for Left<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Left, Some(Symbol::Type), vec![Symbol::Term]),
            "Left Injection",
        )
    }
}
