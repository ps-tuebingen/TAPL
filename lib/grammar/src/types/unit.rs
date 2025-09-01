use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::{language::Language, types::Unit};

impl<Lang> RuleDescribe for Unit<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::UnitTy.into(), "Unit Type")
    }
}
