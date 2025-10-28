use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::Fun};

impl<Lang> GrammarRuleDescribe for Fun<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::fun_ty(), "Function Type")
    }
}
