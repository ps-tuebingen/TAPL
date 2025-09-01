use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::Forall};

impl<Lang> RuleDescribe for Forall<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::forall_ty(Symbol::kind_annot(Symbol::Typevariable)),
            "Universal Type",
        )
    }
}
