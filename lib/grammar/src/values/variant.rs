use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, values::Variant};

impl<Lang> RuleDescribe for Variant<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::variant(Symbol::Value), "Variant")
    }
}
