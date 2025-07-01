use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Product, Type};

impl<Ty> RuleDescribe for Product<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::product_ty(), "Product Type")
    }
}
