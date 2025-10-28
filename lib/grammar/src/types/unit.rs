use crate::{GrammarRuleDescribe, Rule, symbols::Keyword};
use syntax::{language::Language, types::Unit};

impl<Lang> GrammarRuleDescribe for Unit<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::UnitTy.into(), "Unit Type")
    }
}
