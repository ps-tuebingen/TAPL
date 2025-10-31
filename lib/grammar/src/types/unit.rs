use crate::{GrammarRule, GrammarRuleDescribe, symbols::Keyword};
use syntax::{language::Language, types::Unit};

impl<Lang> GrammarRuleDescribe for Unit<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Keyword::UnitTy.into(), "Unit Type")
    }
}
