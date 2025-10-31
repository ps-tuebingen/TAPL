use crate::{GrammarRule, GrammarRuleDescribe, symbols::Keyword};
use syntax::{language::Language, types::Bool};

impl<Lang> GrammarRuleDescribe for Bool<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Keyword::Bool.into(), "Bool")
    }
}
