use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::Product};

impl<Lang> RuleDescribe for Product<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::product_ty(), "Product Type")
    }
}
