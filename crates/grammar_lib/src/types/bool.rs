use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Bool, Type};

impl<Ty> RuleDescribe for Bool<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::term("Bool"), "Bool")
    }
}
