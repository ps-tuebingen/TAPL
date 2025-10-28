use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::Mu};

impl<Lang> GrammarRuleDescribe for Mu<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::mu(), "Mu Type")
    }
}
