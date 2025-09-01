use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::{language::Language, types::Bool};

impl<Lang> RuleDescribe for Bool<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::Bool.into(), "Bool")
    }
}
