use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Assign};

impl<Lang> GrammarRuleDescribe for Assign<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Term, SpecialChar::ColonEq.into(), Symbol::Term].into(),
            "Assignment",
        )
    }
}
