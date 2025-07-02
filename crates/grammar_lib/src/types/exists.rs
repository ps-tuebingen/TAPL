use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Exists, Type};

impl<Ty> RuleDescribe for Exists<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::exists_ty(Symbol::kind_annot(Symbol::Type)),
            "Existential Type",
        )
    }
}
