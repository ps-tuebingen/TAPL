use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, values::Pair};

impl<Lang> GrammarRuleDescribe for Pair<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::brack(Symbol::many(Symbol::Value)), "Pair")
    }
}
