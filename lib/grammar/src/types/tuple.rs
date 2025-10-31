use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, types::Tuple};

impl<Lang> GrammarRuleDescribe for Tuple<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Symbol::paren(Symbol::many(Symbol::Type)), "Tuple Type")
    }
}
