use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Assign};

impl<Lang> GrammarRuleDescribe for Assign<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Term, SpecialChar::ColonEq.into(), Symbol::Term].into(),
            "Assignment",
        )
    }
}
