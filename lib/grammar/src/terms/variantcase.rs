use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::VariantCase};

impl<Lang> RuleDescribe for VariantCase<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![Symbol::Many(Box::new(Symbol::pt(
                Symbol::variant(Symbol::Variable),
                Symbol::Term,
            )))]),
            "Variant Case",
        )
    }
}
