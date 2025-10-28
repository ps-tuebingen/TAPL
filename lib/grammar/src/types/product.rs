use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::Product};

impl<Lang> GrammarRuleDescribe for Product<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::product_ty(), "Product Type")
    }
}
