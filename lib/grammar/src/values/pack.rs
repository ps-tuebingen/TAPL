use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, values::Pack};

impl<Lang> GrammarRuleDescribe for Pack<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            Symbol::brack(Symbol::comma_sep(Symbol::Value, Symbol::Type)),
            "Package",
        )
    }
}
