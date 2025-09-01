use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::OpApp};

impl<Lang> RuleDescribe for OpApp<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::app(Symbol::Type, Symbol::Type),
            "Operator Application",
        )
    }
}
