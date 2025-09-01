use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::{
    language::Language,
    {language::Language, terms::Unit},
};

impl<Lang> RuleDescribe for Unit<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::Unit.into(), "Unit")
    }
}
