use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Sum, Type};

impl<Ty> RuleDescribe for Sum<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::sum_ty(), "Sum Type")
    }
}
