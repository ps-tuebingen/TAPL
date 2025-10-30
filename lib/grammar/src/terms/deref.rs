use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Deref};

impl<Lang> GrammarRuleDescribe for Deref<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![SpecialChar::Exclamation.into(), Symbol::Term].into(),
            "Dereference",
        )
    }
}
