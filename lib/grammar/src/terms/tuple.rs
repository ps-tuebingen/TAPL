use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, terms::Tuple};

impl<Lang> GrammarRuleDescribe for Tuple<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Symbol::paren(Symbol::many(Symbol::Term)), "Tuple")
    }
}
