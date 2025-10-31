use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::RecordProj};

impl<Lang> GrammarRuleDescribe for RecordProj<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Term, SpecialChar::Dot.into(), Symbol::Label].into(),
            "Record Projection",
        )
    }
}
