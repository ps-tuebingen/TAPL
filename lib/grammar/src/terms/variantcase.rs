use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::VariantCase};

impl<Lang> GrammarRuleDescribe for VariantCase<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![Symbol::Many(Box::new(Symbol::pt(
                Symbol::variant(Symbol::Variable),
                Symbol::Term,
            )))]),
            "Variant Case",
        )
    }
}
