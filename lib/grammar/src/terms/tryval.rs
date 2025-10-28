use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::TryWithVal};

impl<Lang> GrammarRuleDescribe for TryWithVal<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::try_catch(), "Try-Catch")
    }
}
