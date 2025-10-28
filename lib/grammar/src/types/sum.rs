use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::Sum};

impl<Lang> GrammarRuleDescribe for Sum<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::sum_ty(), "Sum Type")
    }
}
