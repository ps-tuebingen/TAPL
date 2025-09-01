use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::Variant};

impl<Lang> RuleDescribe for Variant<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::variant(Symbol::Term), "Variant")
    }
}
