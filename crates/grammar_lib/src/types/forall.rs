use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Forall, Type};

impl<Ty> RuleDescribe for Forall<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::forall_ty(Symbol::kind_annot(Symbol::Typevariable)),
            "Universal Type",
        )
    }
}
