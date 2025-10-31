use crate::{GrammarRule, GrammarRuleDescribe, symbols::Keyword};
use syntax::{language::Language, terms::False};

impl<Lang> GrammarRuleDescribe for False<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Keyword::False.into(), "False")
    }
}
