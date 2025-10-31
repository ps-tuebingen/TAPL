use crate::{GrammarRule, GrammarRuleDescribe, symbols::Keyword};
use syntax::{language::Language, terms::Unit};

impl<Lang> GrammarRuleDescribe for Unit<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Keyword::Unit.into(), "Unit")
    }
}
