use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, values::Loc};

impl<Lang> GrammarRuleDescribe for Loc<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Location, "Location")
    }
}
