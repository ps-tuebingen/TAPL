use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Try};

impl<Lang> GrammarRuleDescribe for Try<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::tryt(), "Variant Case")
    }
}
