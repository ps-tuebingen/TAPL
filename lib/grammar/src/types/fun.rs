use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Fun, Type};

impl<Ty> RuleDescribe for Fun<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::fun_ty(), "Function Type")
    }
}
