use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, values::Tuple};

impl<Lang> GrammarRuleDescribe for Tuple<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::paren(Symbol::Many(Box::new(Symbol::Value)))].into(),
            "Tuple",
        )
    }
}
