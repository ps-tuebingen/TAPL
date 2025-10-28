use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::OpApp};

impl<Lang> GrammarRuleDescribe for OpApp<Lang>
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
