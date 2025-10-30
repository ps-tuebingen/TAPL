use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::RecordProj};

impl<Lang> GrammarRuleDescribe for RecordProj<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Term, SpecialChar::Dot.into(), Symbol::Label].into(),
            "Record Projection",
        )
    }
}
