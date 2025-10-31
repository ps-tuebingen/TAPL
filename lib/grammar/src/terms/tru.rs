use crate::{GrammarRule, GrammarRuleDescribe, symbols::Keyword};
use syntax::{language::Language, terms::True};

impl<Lang> GrammarRuleDescribe for True<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Keyword::True.into(), "True")
    }
}
