use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, values::Pack};

impl<Lang> GrammarRuleDescribe for Pack<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::brack(vec![Symbol::Value, SpecialChar::Comma.into(), Symbol::Type]),
            "Package",
        )
    }
}
