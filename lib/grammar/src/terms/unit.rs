use crate::{GrammarRuleDescribe, Rule, symbols::Keyword};
use syntax::{language::Language, terms::Unit};

impl<Lang> GrammarRuleDescribe for Unit<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::Unit.into(), "Unit")
    }
}
