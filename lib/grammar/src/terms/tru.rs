use crate::{GrammarRuleDescribe, Rule, symbols::Keyword};
use syntax::{language::Language, terms::True};

impl<Lang> GrammarRuleDescribe for True<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Keyword::True.into(), "True")
    }
}
