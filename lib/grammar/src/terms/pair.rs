use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, terms::Pair};

impl<Lang> GrammarRuleDescribe for Pair<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Symbol::brack(Symbol::many(Symbol::Term)), "Pair")
    }
}
