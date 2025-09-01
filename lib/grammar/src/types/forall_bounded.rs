use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::ForallBounded};

impl<Lang> RuleDescribe for ForallBounded<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::forall_ty(Symbol::subty_annot(Symbol::Typevariable)),
            "Universal Type",
        )
    }
}
