use crate::{Rule, RuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Projection};

impl<Lang> RuleDescribe for Projection<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(SpecialChar::Number.into()), "Projection")
    }
}
