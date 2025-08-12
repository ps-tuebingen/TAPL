use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{ExistsBounded, Type};

impl<Ty> RuleDescribe for ExistsBounded<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::exists_ty(Symbol::subty_annot(Symbol::Type)),
            "Existential Type",
        )
    }
}
