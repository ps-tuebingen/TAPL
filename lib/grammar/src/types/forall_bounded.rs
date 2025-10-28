use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::ForallBounded};

impl<Lang> GrammarRuleDescribe for ForallBounded<Lang>
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
