use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Cast};

impl<Lang> GrammarRuleDescribe for Cast<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::cast(), "Cast")
    }
}
