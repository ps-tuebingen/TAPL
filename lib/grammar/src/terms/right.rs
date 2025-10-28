use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::Right};

impl<Lang> GrammarRuleDescribe for Right<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Right, Some(Symbol::Type), vec![Symbol::Term]),
            "Right Injection",
        )
    }
}
