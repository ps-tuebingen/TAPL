use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::Sum};

impl<Lang> RuleDescribe for Sum<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::sum_ty(), "Sum Type")
    }
}
