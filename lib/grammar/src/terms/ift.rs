use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::If};

impl<Lang> GrammarRuleDescribe for If<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::ift(), "If")
    }
}
