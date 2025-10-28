use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Loc};

impl<Lang> GrammarRuleDescribe for Loc<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Location, "Location")
    }
}
