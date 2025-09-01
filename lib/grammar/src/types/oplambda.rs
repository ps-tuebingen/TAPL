use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::OpLambda};

impl<Lang> RuleDescribe for OpLambda<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::Kind, Symbol::Type),
            "Operator Abstraction",
        )
    }
}
