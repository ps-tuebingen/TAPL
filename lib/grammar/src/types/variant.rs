use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::Variant};

impl<Lang> GrammarRuleDescribe for Variant<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::variant(Symbol::Type), "Variant Type")
    }
}
