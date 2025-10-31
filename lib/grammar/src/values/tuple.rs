use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, values::Tuple};

impl<Lang> GrammarRuleDescribe for Tuple<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::paren(Symbol::Many(Box::new(Symbol::Value)))].into(),
            "Tuple",
        )
    }
}
