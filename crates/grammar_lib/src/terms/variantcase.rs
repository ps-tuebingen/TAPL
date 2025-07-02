use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, VariantCase};

impl<T> RuleDescribe for VariantCase<T>
where
    T: Term,
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
