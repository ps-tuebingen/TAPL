use crate::{GrammarRuleDescribe, Rule, symbols::Keyword};
use syntax::{language::Language, values::False};

impl<Lang> GrammarRuleDescribe for False<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::False.into(), "False")
    }
}
