use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::{language::Language, values::False};

impl<Lang> RuleDescribe for False<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::False.into(), "False")
    }
}
