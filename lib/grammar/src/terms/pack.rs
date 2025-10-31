use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, terms::Pack};

impl<Lang> GrammarRuleDescribe for Pack<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            Symbol::brack(Symbol::comma_sep(Symbol::Term, Symbol::Type)),
            "Package",
        )
    }
}
