use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, values::TyLambda};

impl<Lang> GrammarRuleDescribe for TyLambda<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::kind_annot(Symbol::Variable), Symbol::Value),
            "Type Abstraction",
        )
    }
}
