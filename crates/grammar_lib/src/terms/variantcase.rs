use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, VariantCase};

impl<T> RuleDescribe for VariantCase<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![Symbol::pt("label", 1), Symbol::pt("...", 0)]),
            "Variant Case",
        )
    }
}
