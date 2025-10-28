use crate::{GrammarRuleDescribe, Rule, symbols::Keyword};
use syntax::{language::Language, terms::False};

impl<Lang> GrammarRuleDescribe for False<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::False.into(), "False")
    }
}
