use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, values::Variant};

impl<Lang> GrammarRuleDescribe for Variant<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::variant(Symbol::Value), "Variant")
    }
}
