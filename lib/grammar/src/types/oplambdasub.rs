use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::OpLambdaSub};

impl<Lang> RuleDescribe for OpLambdaSub<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::Type, Symbol::Type),
            "Operator Abstraction",
        )
    }
}
