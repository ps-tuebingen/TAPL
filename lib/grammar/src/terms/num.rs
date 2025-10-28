use crate::{GrammarRuleDescribe, Rule, symbols::SpecialChar};
use syntax::{language::Language, terms::Num};

impl<Lang> GrammarRuleDescribe for Num<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Number.into(), "Number")
    }
}
