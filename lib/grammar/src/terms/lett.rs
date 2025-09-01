use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::Let};

impl<Lang> RuleDescribe for Let<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::lett(), "Let Binding")
    }
}
