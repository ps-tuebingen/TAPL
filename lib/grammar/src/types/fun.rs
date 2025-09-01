use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::Fun};

impl<Lang> RuleDescribe for Fun<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::fun_ty(), "Function Type")
    }
}
