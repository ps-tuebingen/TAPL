use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, values::Loc};

impl<Lang> GrammarRuleDescribe for Loc<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Symbol::Location, "Location")
    }
}
