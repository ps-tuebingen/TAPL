use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, values::Pair};

impl<Lang> GrammarRuleDescribe for Pair<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Symbol::brack(Symbol::many(Symbol::Value)), "Pair")
    }
}
