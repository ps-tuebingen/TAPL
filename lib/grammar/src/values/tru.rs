use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::{language::Language, values::True};

impl<Lang> RuleDescribe for True<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::True.into(), "True")
    }
}
