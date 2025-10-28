use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Let};

impl<Lang> GrammarRuleDescribe for Let<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::lett(), "Let Binding")
    }
}
