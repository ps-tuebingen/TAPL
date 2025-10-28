use crate::{GrammarRuleDescribe, Rule, symbols::Keyword};
use syntax::{language::Language, types::Bool};

impl<Lang> GrammarRuleDescribe for Bool<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::Bool.into(), "Bool")
    }
}
