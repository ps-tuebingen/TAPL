use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::ExistsBounded};

impl<Lang> GrammarRuleDescribe for ExistsBounded<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::exists_ty(Symbol::subty_annot(Symbol::Type)),
            "Existential Type",
        )
    }
}
