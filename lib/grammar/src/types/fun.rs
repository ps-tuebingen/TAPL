use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Fun};

impl<Lang> GrammarRuleDescribe for Fun<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Type, SpecialChar::Arrow.into(), Symbol::Type].into(),
            "Function Type",
        )
    }
}
