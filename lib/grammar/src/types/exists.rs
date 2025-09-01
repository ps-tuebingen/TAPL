use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::Exists};

impl<Lang> RuleDescribe for Exists<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::exists_ty(Symbol::kind_annot(Symbol::Type)),
            "Existential Type",
        )
    }
}
