use crate::{GrammarRuleDescribe, Rule, symbols::SpecialChar};
use syntax::{language::Language, values::Num};

impl<Lang> GrammarRuleDescribe for Num<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Number.into(), "Number")
    }
}
