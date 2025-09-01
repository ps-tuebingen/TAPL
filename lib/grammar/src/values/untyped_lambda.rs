use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, values::UntypedLambda};

impl<Lang> RuleDescribe for UntypedLambda<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::lam_untyped(Symbol::Term), "Lambda Abstraction")
    }
}
