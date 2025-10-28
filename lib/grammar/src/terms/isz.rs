use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::Keyword};
use syntax::{language::Language, terms::IsZero};

impl<Lang> GrammarRuleDescribe for IsZero<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::IsZero, None, vec![Symbol::Term]),
            "IsZero",
        )
    }
}
