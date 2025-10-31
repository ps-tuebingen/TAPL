use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, values::Pack};

impl<Lang> GrammarRuleDescribe for Pack<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::brack(Symbol::comma_sep(Symbol::Value, Symbol::Type)),
            "Package",
        )
    }
}
