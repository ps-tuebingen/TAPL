use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::OpLambda};

impl<Lang> GrammarRuleDescribe for OpLambda<Lang>
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
