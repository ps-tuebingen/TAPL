use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::Variant};

impl<Lang> RuleDescribe for Variant<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::variant(Symbol::Type), "Variant Type")
    }
}
