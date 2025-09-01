use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::TyLambda};

impl<Lang> RuleDescribe for TyLambda<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::kind_annot(Symbol::Variable), Symbol::Term),
            "Type Abstraction",
        )
    }
}
